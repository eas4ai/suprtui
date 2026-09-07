//! Clipboard lifecycle (SYS-003).
//!
//! Ports the `host.zig` operation lifecycle — start, poll, result,
//! destroy — with cancellation, over an injected backend so the
//! whole state machine runs headless. The reference spawns one
//! thread per operation and fans out to X11, Wayland, Windows, and
//! macOS backends; this port keeps the observable lifecycle and
//! replaces the platform fan-out with a [`Backend`] trait. Tests
//! drive it with [`LoopbackBackend`] (and deferred script backends
//! in the suite), never a display server.

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Statuses (mirror `host.zig`).
// ---------------------------------------------------------------------------

/// Operation state. `InvalidHandle` reports polling an id the
/// service does not hold, as in the reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationStatus {
    Pending,
    Read,
    Empty,
    Written,
    Cleared,
    Unsupported,
    Cancelled,
    TimedOut,
    LimitExceeded,
    Failed,
    InvalidHandle,
}

/// Outcome of [`Service::cancel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelOutcome {
    Requested,
    AlreadyTerminal,
    InvalidHandle,
}

/// Outcome of [`Service::destroy`]. Destroying a pending operation
/// is refused (`NotReady`); cancel first, as in the reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestroyOutcome {
    Destroyed,
    NotReady,
    InvalidHandle,
}

/// Why [`Service::start`] refused an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartError {
    ShuttingDown,
    LimitExceeded,
    InvalidArgument,
}

/// Why [`Service::result`] has no bytes to hand out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultError {
    InvalidHandle,
    NotReady,
    Cancelled,
    Failed(OperationStatus),
}

/// Why [`Service::copy_into`] refused the copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyError {
    BufferTooSmall,
    InvalidHandle,
    NotReady,
    Cancelled,
    Failed(OperationStatus),
}

/// What to read, write, or clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationKind {
    Read,
    Write,
    Clear,
}

// ---------------------------------------------------------------------------
// Backend seam.
// ---------------------------------------------------------------------------

/// What the backend was asked to do.
#[derive(Debug, Clone)]
pub struct BackendRequest {
    pub kind: OperationKind,
    pub payload: Vec<u8>,
    pub max_bytes: usize,
}

/// One backend step. `Pending` keeps the operation pending; the
/// service polls again later, so completion is never reported
/// early.
#[derive(Debug, Clone)]
pub enum BackendStep {
    Pending,
    Complete(BackendOutcome),
}

/// A finished backend step.
#[derive(Debug, Clone)]
pub enum BackendOutcome {
    Read(Vec<u8>),
    Empty,
    Written,
    Cleared,
    Unsupported,
    LimitExceeded,
    Failed,
}

/// Platform clipboard behind the lifecycle. Implementations must be
/// callable headless when tests need them; the suite uses
/// [`LoopbackBackend`].
pub trait Backend {
    fn step(&mut self, request: &BackendRequest, polls: u64) -> BackendStep;
}

/// In-memory backend: reads return the stored bytes (`Empty` when
/// none), writes replace them, clears drop them. Enforces
/// `max_bytes`, reporting `LimitExceeded` past the cap.
#[derive(Debug, Default)]
pub struct LoopbackBackend {
    store: Vec<u8>,
}

impl LoopbackBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: &[u8]) -> Self {
        Self {
            store: seed.to_vec(),
        }
    }
}

impl Backend for LoopbackBackend {
    fn step(&mut self, request: &BackendRequest, _polls: u64) -> BackendStep {
        match request.kind {
            OperationKind::Read => {
                if self.store.is_empty() {
                    BackendStep::Complete(BackendOutcome::Empty)
                } else {
                    BackendStep::Complete(BackendOutcome::Read(self.store.clone()))
                }
            }
            OperationKind::Write => {
                if request.payload.len() > request.max_bytes {
                    BackendStep::Complete(BackendOutcome::LimitExceeded)
                } else {
                    self.store = request.payload.clone();
                    BackendStep::Complete(BackendOutcome::Written)
                }
            }
            OperationKind::Clear => {
                self.store.clear();
                BackendStep::Complete(BackendOutcome::Cleared)
            }
        }
    }
}

/// Backend that always reports the platform missing. Models running
/// with no display server.
#[derive(Debug, Default)]
pub struct UnsupportedBackend;

impl Backend for UnsupportedBackend {
    fn step(&mut self, _request: &BackendRequest, _polls: u64) -> BackendStep {
        BackendStep::Complete(BackendOutcome::Unsupported)
    }
}

// ---------------------------------------------------------------------------
// Service.
// ---------------------------------------------------------------------------

/// Per-start options.
#[derive(Debug, Clone, Copy)]
pub struct StartOptions {
    pub timeout: Option<Duration>,
    pub max_bytes: usize,
}

impl Default for StartOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            max_bytes: 1024 * 1024,
        }
    }
}

struct Operation {
    kind: OperationKind,
    payload: Vec<u8>,
    max_bytes: usize,
    status: OperationStatus,
    result: Vec<u8>,
    cancel_requested: bool,
    deadline: Option<Instant>,
    polls: u64,
}

/// Caller-owned clipboard service. Operation ids are scoped to
/// their service; every pool, backend, and operation lives in the
/// caller's structs, never in process globals.
pub struct Service<B: Backend> {
    backend: B,
    operations: HashMap<u64, Operation>,
    next_id: u64,
    max_operations: usize,
    shutting_down: bool,
}

impl<B: Backend> Service<B> {
    pub fn new(backend: B) -> Self {
        Self::with_capacity(backend, 64)
    }

    pub fn with_capacity(backend: B, max_operations: usize) -> Self {
        Self {
            backend,
            operations: HashMap::new(),
            next_id: 1,
            max_operations,
            shutting_down: false,
        }
    }

    /// Refuse further starts. In-flight operations still run.
    pub fn shutdown(&mut self) {
        self.shutting_down = true;
    }

    /// Start a read, write, or clear. Returns the operation id.
    pub fn start(
        &mut self,
        kind: OperationKind,
        payload: &[u8],
        options: StartOptions,
    ) -> Result<u64, StartError> {
        if self.shutting_down {
            return Err(StartError::ShuttingDown);
        }
        if self.operations.len() >= self.max_operations {
            return Err(StartError::LimitExceeded);
        }
        if options.max_bytes == 0 {
            return Err(StartError::InvalidArgument);
        }
        let id = self.next_id;
        self.next_id += 1;
        let deadline = options.timeout.map(|t| Instant::now() + t);
        self.operations.insert(
            id,
            Operation {
                kind,
                payload: payload.to_vec(),
                max_bytes: options.max_bytes,
                status: OperationStatus::Pending,
                result: Vec::new(),
                cancel_requested: false,
                deadline,
                polls: 0,
            },
        );
        Ok(id)
    }

    /// Drive one operation. Never reports completion early: a
    /// backend that is still working keeps the status pending, and
    /// a cancelled operation reports cancelled without consulting
    /// the backend again.
    pub fn poll(&mut self, id: u64) -> OperationStatus {
        let operation = match self.operations.get_mut(&id) {
            Some(operation) => operation,
            None => return OperationStatus::InvalidHandle,
        };
        if operation.status != OperationStatus::Pending {
            return operation.status;
        }
        if operation.cancel_requested {
            operation.status = OperationStatus::Cancelled;
            return operation.status;
        }
        if operation
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            operation.status = OperationStatus::TimedOut;
            return operation.status;
        }
        let request = BackendRequest {
            kind: operation.kind,
            payload: operation.payload.clone(),
            max_bytes: operation.max_bytes,
        };
        operation.polls += 1;
        match self.backend.step(&request, operation.polls) {
            BackendStep::Pending => OperationStatus::Pending,
            BackendStep::Complete(outcome) => {
                let status = match outcome {
                    BackendOutcome::Read(bytes) => {
                        operation.result = bytes;
                        OperationStatus::Read
                    }
                    BackendOutcome::Empty => OperationStatus::Empty,
                    BackendOutcome::Written => {
                        operation.result = operation.payload.clone();
                        OperationStatus::Written
                    }
                    BackendOutcome::Cleared => OperationStatus::Cleared,
                    BackendOutcome::Unsupported => OperationStatus::Unsupported,
                    BackendOutcome::LimitExceeded => OperationStatus::LimitExceeded,
                    BackendOutcome::Failed => OperationStatus::Failed,
                };
                // A cancel that raced completion still wins: the
                // reference drops late results the same way.
                operation.status = if operation.cancel_requested {
                    OperationStatus::Cancelled
                } else {
                    status
                };
                operation.status
            }
        }
    }

    /// Cancel a pending operation. A cancelled operation never
    /// delivers a result.
    pub fn cancel(&mut self, id: u64) -> CancelOutcome {
        match self.operations.get_mut(&id) {
            None => CancelOutcome::InvalidHandle,
            Some(operation) if operation.status != OperationStatus::Pending => {
                CancelOutcome::AlreadyTerminal
            }
            Some(operation) => {
                operation.cancel_requested = true;
                operation.status = OperationStatus::Cancelled;
                CancelOutcome::Requested
            }
        }
    }

    /// Read a finished operation's bytes: read data, or the echoed
    /// payload of a completed write. `Empty` and `Cleared` yield an
    /// empty slice; every other non-data terminal state is an
    /// error, and a cancelled operation delivers nothing.
    pub fn result(&self, id: u64) -> Result<&[u8], ResultError> {
        match self.operations.get(&id) {
            None => Err(ResultError::InvalidHandle),
            Some(operation) => match operation.status {
                OperationStatus::Pending => Err(ResultError::NotReady),
                OperationStatus::Cancelled => Err(ResultError::Cancelled),
                OperationStatus::Read
                | OperationStatus::Empty
                | OperationStatus::Written
                | OperationStatus::Cleared => Ok(&operation.result),
                status => Err(ResultError::Failed(status)),
            },
        }
    }

    /// Copy a finished operation's bytes into a caller buffer, as
    /// the reference copy-to-buffer call does.
    pub fn copy_into(&self, id: u64, buffer: &mut [u8]) -> Result<usize, CopyError> {
        let bytes = match self.result(id) {
            Ok(bytes) => bytes,
            Err(ResultError::InvalidHandle) => return Err(CopyError::InvalidHandle),
            Err(ResultError::NotReady) => return Err(CopyError::NotReady),
            Err(ResultError::Cancelled) => return Err(CopyError::Cancelled),
            Err(ResultError::Failed(status)) => return Err(CopyError::Failed(status)),
        };
        if buffer.len() < bytes.len() {
            return Err(CopyError::BufferTooSmall);
        }
        buffer[..bytes.len()].copy_from_slice(bytes);
        Ok(bytes.len())
    }

    /// Destroy a finished operation and free it. Pending
    /// operations are refused; cancel first.
    pub fn destroy(&mut self, id: u64) -> DestroyOutcome {
        match self.operations.get(&id) {
            None => DestroyOutcome::InvalidHandle,
            Some(operation) if operation.status == OperationStatus::Pending => {
                DestroyOutcome::NotReady
            }
            Some(_) => {
                self.operations.remove(&id);
                DestroyOutcome::Destroyed
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Lifecycle unit tests (the runner also drives tests/sys_clipboard.rs).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod lifecycle {
    use super::*;

    #[test]
    fn terminal_poll_repeats_status() {
        let mut service = Service::new(LoopbackBackend::new());
        let id = service
            .start(OperationKind::Write, b"hi", StartOptions::default())
            .unwrap();
        assert_eq!(service.poll(id), OperationStatus::Written);
        assert_eq!(service.poll(id), OperationStatus::Written);
    }

    #[test]
    fn cancel_after_completion_is_already_terminal() {
        let mut service = Service::new(LoopbackBackend::new());
        let id = service
            .start(OperationKind::Clear, b"", StartOptions::default())
            .unwrap();
        assert_eq!(service.poll(id), OperationStatus::Cleared);
        assert_eq!(service.cancel(id), CancelOutcome::AlreadyTerminal);
    }
}

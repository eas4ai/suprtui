# System (SYS)

Status: Agreed 2026-09-07.
Prefix: SYS.

Reference: `reference/opentui-0.5.11/packages/native/src/handles.zig`,
`reference/opentui-0.5.11/packages/native/src/event-bus.zig`,
`reference/opentui-0.5.11/packages/native/src/event-emitter.zig`,
`reference/opentui-0.5.11/packages/native/src/clipboard/host.zig`,
`reference/opentui-0.5.11/packages/native/src/logger.zig`,
`reference/opentui-0.5.11/packages/native/src/mem-registry.zig`,
`reference/opentui-0.5.11/packages/native/src/native-span-feed.zig`,
`reference/opentui-0.5.11/packages/native/src/link.zig`,
`reference/opentui-0.5.11/packages/native/src/split-scrollback.zig`, and
the fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/handles_test.zig`,
`event-emitter_test.zig`, `native-span-feed_test.zig`,
`link_test.zig`, and `split-scrollback_test.zig`. The crate exposes
this domain as `src/sys.rs`. It also owns the cross-cutting rules
from the keystone: error handling, shared-state bans, and the
verification commands.

[SYS-001]
Object identity MUST use Rust ownership, and the crate MUST NOT keep
a process-global handle registry.
Falsifier: two engine instances observe each other's objects, or a
global registry exists in `src/`.
Mechanism: `cargo test -p suprtui sys::instance_isolation` plus
review of `src/`.

[SYS-002]
Event sinks MUST receive posted events in order, and destroying a
sink MUST stop its delivery.
Falsifier: an event arrives out of order, or a destroyed sink
receives another event.
Mechanism: `cargo test -p suprtui sys::event_bus`.

[SYS-003]
Clipboard read, write, and clear operations MUST follow the
start, poll, result, destroy lifecycle with cancellation, and the
state machine MUST be drivable by an injected backend so tests run
without a display server.
Falsifier: polling reports completion early, cancelling still
delivers a result, or the lifecycle cannot run headless.
Mechanism: `cargo test -p suprtui sys::clipboard_lifecycle` with a
loopback backend.

[SYS-004]
The link pool MUST intern URLs up to 512 bytes with reference
counting, MUST reject longer URLs, and MUST scope ids to their pool.
Falsifier: a 513-byte URL is accepted, or an id from another pool
resolves.
Mechanism: `cargo test -p suprtui sys::link_pool`.

[SYS-005]
The span feed MUST deliver written byte spans in order, MUST keep
atomic writes contiguous, and MUST deliver each drained span exactly
once.
Falsifier: a drained span reappears, or an atomic write arrives
split around another write.
Mechanism: `cargo test -p suprtui sys::span_feed` over ported feed
vectors.

[SYS-006]
Split-scrollback accounting MUST clamp the render offset to
published rows and MUST follow the reference newline, scroll, and
snapshot formulas.
Falsifier: any ported scrollback vector disagrees with the module.
Mechanism: `cargo test -p suprtui sys::split_scrollback` over ported
vectors.

[SYS-007]
Logging MUST honor err, warn, info, and debug levels through a
caller-supplied sink, and MUST drop messages when no sink is set
without panicking.
Falsifier: a debug message is delivered while the level is err, or
logging without a sink panics.
Mechanism: `cargo test -p suprtui sys::logging`.

[SYS-008]
Every public fallible function MUST return a `Result` with a crate
error type, and panics MUST be reserved for documented caller-contract
violations only.
Falsifier: any public function panics on malformed input, or returns
an untyped error.
Mechanism: `cargo test -p suprtui sys::error_discipline` with a
malformed-input corpus, plus `cargo clippy -p suprtui`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review removed the reference's global
handle registry deliberately: it existed to serve the C interface,
which this crate does not expose, and Rust ownership replaces it
(SYS-001 names the observable difference). It challenged SYS-003's
headless demand and kept it, since platform clipboards cannot run in
CI and an untestable lifecycle is an unfinished requirement. It
checked SYS-004's 512-byte limit against the reference constant and
SYS-006's formulas against the reference source line by line. No spec
lint binary exists in this repository yet, so the one-obligation rule
was checked by hand.

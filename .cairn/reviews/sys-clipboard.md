# Review: sys-clipboard

commit: fbc53bb9743aa9b08b52301f05bd670c2e29dc9e
findings:
  - closed: read, write, and clear each complete start-poll-result-destroy against the loopback backend with 12 vectors; deferred script backend proves polling never reports completion early; cancel suppresses the result and destroy refuses pending operations; full suite green, crate-wide clippy zero warnings, fmt clean, no `unsafe`, no new dependencies
  - closed: SYS-003 src/ review: the service, backends, and operations are caller-owned generics (`Service<B: Backend>`, no `Box<dyn>`); no process-global state — `grep static src/clipboard.rs` finds only doc text; operation ids are scoped to their service instance
  - closed: SPEC-022 faults discriminate exactly (early-complete fails only no_early_completion, cancel noop fails only cancel_suppresses_result, destroy-frees-pending fails only destroy_pending_is_not_ready); all faults reverted in-session with the file verified byte-identical to its pre-fault copy
  - closed: platform scope is honest — the reference X11/Wayland/Windows/macOS workers are threads plus OS APIs outside safe std-only Rust; the `Backend` trait plus `UnsupportedBackend` is the declared seam, and every lifecycle claim is headless-tested

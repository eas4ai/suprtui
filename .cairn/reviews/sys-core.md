# Review: sys-core

commit: 22387a35b836e400bb4d5bfcfd58b88e88095628
findings:
  - closed: ordered event bus with destroyable sinks, level-gated caller-owned logging, direct-port split-scrollback accounting, chunked span feed with atomic contiguity and exactly-once drains, link-pool limit/scoping vectors, six ported scrollback vectors, and a malformed-input corpus across public APIs land with 7 vectors; full suite green, sys-owned code clippy-clean with no `unsafe`, no new dependencies
  - closed: SYS-001 src/ review: no process-global registries exist — `grep static src/` finds only a `&'static str` return type; every pool, sink, logger, feed, engine, tree, and terminal is caller-owned; cross-instance independence is asserted per type in req_001
  - closed: SPEC-022 faults discriminate per requirement (shared callback cell exactly SYS-001, destroy noop exactly SYS-002, URL-limit fault fails SYS-004 and SYS-008 together since the corpus re-pins the same limit, drain-without-removal exactly SYS-005, unclamped offset exactly SYS-006, ignored level exactly SYS-007, unguarded device select exactly SYS-008); all faults reverted in-session
  - open: close blocked on scope answer loop-035-3 — committed clippy fix d5b4655 (vec-to-array repeat in tests/media_image.rs, behavior-identical) postdates sys-core activation; restored to activation content per the tool's demand, which returns 2 pre-existing clippy warnings in that file; sys-core's own code stays warning-free
  - open: fresh clones cannot run tests/media_image.rs — its display-p3 fixture is an include_str into untracked reference/ (217M, ignored); recommend inlining the 593-byte fixture once the scope lock clears

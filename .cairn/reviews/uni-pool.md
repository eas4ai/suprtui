# Review: uni-pool

commit: e2a8c2fcd1b5ce0022166ab93a2a248deac560d0
findings:
  - closed: pool and tracker land with 66 integration tests plus the spec-literal `uni::pool_isolation` unit test; full workspace 420 pass, clippy zero warnings, fmt clean, no `unsafe` in the crate
  - closed: all reference pool, unowned-aliasing (pointer-identity holds verbatim), interning, exhaustion, and tracker vectors ported; the three global-pool cases are excluded by UNI-008 design, covered instead by two-level isolation tests
  - closed: unowned stores `&'a [u8]` (true aliasing, compiler-checked lifetime) and the tracker shares its pool via `Rc<RefCell<..>>` (simultaneous trackers port verbatim); both deviations are safety-preserving and recorded in the commitment
  - closed: SPEC-022 fault demonstration showed precision (disabled generation check failed exactly UNI-008 while the width and segments suites stayed green); tree restored to all-pass after

# Review: sys-small

commit: 2582733048dbd6d9f177651e8853bfbf61657ecd
findings:
  - closed: listeners attach per event, detach by stable slot id, and fire in registration order; detached and unknown ids stay silent; emission with no listeners is a no-op — 4 vectors plus order unit test
  - closed: file logger appends exactly the at-and-above-level lines across logger instances, drops below-level silently, and reports directory-as-file I/O failure instead of panicking — 3 vectors plus gating unit test, temp files removed
  - closed: SPEC-022 faults discriminate exactly (detach noop, removed gate); both reverted byte-clean
  - closed: full suite green, clippy zero, fmt clean, no `unsafe`, no `static`, no new dependencies

# Review: render-core

commit: d9dc82c23542bc9f8c86aa14dfad527cec723cc0
findings:
  - closed: double-buffered diff publish, unchanged skip, cursor move/style tracking, failed-frame rollback, exact memory capture, and frame stats land with 6 spec-literal unit tests plus 6 integration vectors; full suite green (492 tests), clippy zero warnings, fmt clean, no `unsafe`
  - closed: immediate-mode contract verified the hard way: the first test draft omitted per-frame redraws and failed, confirming the next buffer is scratch cleared after every frame; the review keeps the redraw comments in the tests
  - closed: SPEC-022 faults precise per requirement except the emission path, honestly joint on REN-001/REN-004/REN-005; the coarse empty-frame fault jointly failed REN-002/REN-003/REN-005 with a precise truncating replacement failing exactly REN-005; all faults reverted in-session
  - closed: evidence recorded 20260907T173624737Z, all six pass by line, after ok on loop-035-2 acknowledged the restored scope history; the flag stood truthfully until then and this finding resolves it

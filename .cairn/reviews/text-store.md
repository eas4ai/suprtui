# Review: text-store

commit: 29435b0d63cb62942fdfa9d671275be237b3a34f
findings:
  - closed: byte-offset storage with insert, delete, split, and plain-text readback, span shift/stretch/clamp/drop, snapshot undo/redo with redo cleared on new edits, and view dirt under an advancing epoch land with 4 spec-literal unit tests plus 4 integration vectors; full suite green, clippy zero warnings, fmt clean, no `unsafe`
  - closed: the delete-overlap math was derived wrong once (right-overlap spans kept deleted text) and corrected before any test ran; the unit tests then caught three hand-arithmetic slips in expectations, all fixed in the tests, never the implementation
  - closed: SPEC-022 faults precise per requirement except the split, honestly joint on TXT-001/TXT-004 first with a precise boundary-char replacement failing exactly TXT-001; all faults reverted in-session
  - closed: declaration lagged implementation by two commits (mechanism-first momentum under a stale Current), caught by the scope guard naming all three undeclared paths at check time; declaring then checking recorded cleanly with no restore needed

# Review: text-view

commit: 7db41dd54848683246443efc262121a0dc9a95d2
findings:
  - closed: word/char wrapping on uni breaks, word and line selection with exact extraction, cluster-safe cursor edits with line crossing, reference-keyed highlight overlays, substring syntax styles, and total bounds safety land with 6 spec-literal unit tests plus 6 integration vectors; full suite green, clippy zero warnings, fmt clean, no `unsafe`
  - closed: byte-form discipline enforced the hard way: ambiguous composed é literals replaced by explicit escapes after a mid-acute boundary rejection exposed a wrong cluster map; arrow expectations corrected to true cluster starts (1, 4, 5)
  - closed: one real implementation bug found by integration (move_vertical mixed the target line start with the origin cursor and panicked); column now comes from the origin line
  - closed: SPEC-022 faults fail exactly their requirement, all six (width-only word wrap, unordered selection, char-wise backspace, clear-all highlight removal, mistagged syntax spans, skipped boundary check); all faults reverted in-session

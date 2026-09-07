# Review: uni-width

commit: 789cc361675dcb7e2d8ace8f5a1b9e6a79be1137
findings:
  - closed: full tables and ported suites landed in 789cc36 (UCD 17.0.0 verbatim extracts, 272 ported tests incl. 3907-assertion map, multilingual consistency); seed default is gone
  - closed: merged unicode_wide/spacing-mark arms are observably identical (pure shared body); per-requirement SPEC-022 precision re-verified via the runner after the merge
  - closed: method parameter is accepted-but-unused at this layer, matching the reference where code-point width takes no method and the method enters at segmentation (uni-segments)
  - closed: tab byte reports 0 here; tab-stop expansion belongs to UNI-005 (uni-segments), not this commitment
  - closed: byte-level invalid UTF-8 handling belongs to UNI-007 (uni-segments); this layer takes decoded code points
  - closed: SPEC-022 fault demonstration showed per-requirement precision (U+231A removal failed exactly UNI-002 and UNI-003)

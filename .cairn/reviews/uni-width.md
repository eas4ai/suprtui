# Review: uni-width

commit: 5a5170949d481c1ca4bd17c7e0aa01d85cd68aef
findings:
  - open: full Unicode property tables and ported vector suites from Done-when are outstanding; the seed proves the harness on a verified subset only
  - closed: method parameter is accepted-but-unused at this layer, matching the reference where code-point width takes no method and the method enters at segmentation (uni-segments)
  - closed: tab byte reports 0 here; tab-stop expansion belongs to UNI-005 (uni-segments), not this commitment
  - closed: byte-level invalid UTF-8 handling belongs to UNI-007 (uni-segments); this layer takes decoded code points
  - closed: SPEC-022 fault demonstration showed per-requirement precision (U+231A removal failed exactly UNI-002 and UNI-003)

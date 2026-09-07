# Review: uni-segments

commit: 9f6f2eaf3547084d7181d6c0007641005a63f9ca
findings:
  - closed: 81 tests land in 9f6f2ea (65 ported vectors plus 16 hand-ported golden/builder/random/corpus/kind/packing tests); full workspace 353 pass, clippy zero warnings, fmt clean
  - closed: golden-table extraction first misaligned cases (block-split on the wrong region; CRLF row read [2,1] against true {2}); caught by the suite, re-extracted from the three true golden arrays with a per-array .name-count completeness assert in scripts/extract-utf8-goldens.py; 15 line-break plus 11 tab-stop plus 24 layout-break rows now pass verbatim
  - closed: seven hand-written expectations were miscounted on first write (tab byte, chunk offsets, wrap columns, mid-sequence decode kind, word-class of trailing punctuation, tab pair); each corrected against the reference algorithm or byte layout, never against the implementation output alone
  - closed: transcription audit sampled ported vectors across every area (isAsciiOnly, wrap, pos, split, prev-grapheme, line builders, Thai, no_zwj) against the reference test fns; all values match exactly
  - closed: SPEC-022 fault demonstration showed per-requirement precision (no_zwj split failed exactly UNI-004, tab shift exactly UNI-005, CRLF kind flip exactly UNI-006, truncated-sequence accept exactly UNI-007, packing shift exactly UNI-009); tree restored to all-pass after

# Unicode tables from UCD 17.0.0 with unicode-segmentation and unicode-properties crates

Level: Judged
Decided by: shawn
Rests on: UNI-001 UNI-002 UNI-003
Would be wrong if: ported vectors disagree with the chosen data

## Decision

Width property data comes from UCD 17.0.0 files extracted verbatim (East_Asian_Width W/F, Grapheme_Cluster_Break SpacingMark/Prepend, Default_Ignorable_Code_Point); general categories from the unicode-properties crate; segmentation from the unicode-segmentation crate with manual U+FFFD joints and no_zwj ZWJ splits. The 3907-assertion ported map plus targeted vectors arbitrate every disagreement; a vector failure reopens this choice.

## Realized by

- 789cc361675dcb7e2d8ace8f5a1b9e6a79be1137 Implement full uni width tables with ported reference vectors

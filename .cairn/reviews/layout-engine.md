# Review: layout-engine

commit: f5b330251005ef4a24e8ad78368007c97bea15a7
findings:
  - closed: taffy-backed flex tree with classical Yoga defaults (column direction, unrounded point scale one), caller-owned measure targets with call counts, and absolute nested positions land with 4 ported vectors plus feature coverage for wrap, justify, gaps, padding, margins, min/max, and alignment; full suite green, clippy zero warnings, fmt clean, no `unsafe`, one new dependency (`taffy`, pure Rust)
  - closed: one real API hazard found by integration: taffy panics on stale node ids instead of erroring, so the wrapper tracks liveness and reports InvalidNode; two test expectations corrected for the Stretch default (min-width and measured height)
  - closed: SPEC-022 faults fail exactly their requirement, all four (center justification exactly LAY-001, column default exactly LAY-002, measure bypass exactly LAY-003, dropped ancestor offsets exactly LAY-004); one probe initially broke compilation rather than discriminating and was redone honestly; all faults reverted in-session

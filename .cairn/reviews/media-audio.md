# Review: media-audio

commit: 011c05e866e31fd433d3995e7862024ac81e5247
findings:
  - closed: deterministic engine with fallible lifecycle, accounting PCM streams with volume/pan/group, device listing with range-checked selection, order-preserving capture with matching stats, and id-addressed sounds with voices, numeric groups, clamped volumes, mixed output, and tap capture land with 5 vectors; full suite green, clippy zero warnings, fmt clean, no `unsafe`, no new dependencies
  - closed: two reference behaviors corrected during implementation: groups are numeric ids with group 0 default (not names), and volumes clamp to [0, 4] (not [0, 1]); taffy-style liveness guarding reports InvalidNode instead of panicking on stale ids
  - closed: SPEC-022 faults fail exactly their requirement, all five (device gate exactly MED-003, close gate exactly MED-004, selection bound exactly MED-005, drop-oldest overflow exactly MED-006, id reuse exactly MED-007); the first MED-005 probe slipped through and exposed a missing boundary assertion, which was added before re-probing honestly; all faults reverted in-session

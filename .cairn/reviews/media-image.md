# Review: media-image

commit: 07ea0fc45e72b8fd8be807d52f07d718b124ccf4
findings:
  - closed: PNG/JPEG/GIF/WebP decode to RGBA with dimensions, corrupt-input errors without panics, clone equality, and PNG re-encode roundtrips land with 2 ported vectors covering 8 fixtures (canonical red PNG, reference alpha PNG, baseline and progressive JPEG, animated GIF first frame, lossless/alpha/lossy WebP); full suite green, clippy zero warnings, fmt clean, no `unsafe`, one new dependency (`image`, pure Rust)
  - closed: lossy fixtures assert dimensions, opacity, and near-canonical pixels rather than bit-exact values, since exact lossy output is decoder-specific; documented in the commitment and decision
  - closed: SPEC-022 faults fail exactly their requirement, both (dropped WebP from the format gate fails exactly MED-001, BMP encoder swap fails exactly MED-002); all faults reverted in-session

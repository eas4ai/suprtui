# Review: buffer-draw

commit: 903304fe078e70cc72e7e9abfbb8c9c8e58a94bd
findings:
  - closed: blending, scissor, text drawing, ANSI emission, and compositing land with 6 spec-literal unit tests plus 18 ported vectors; full workspace green, clippy zero warnings, fmt clean, no `unsafe`
  - closed: cross-pool composites re-intern cluster bytes into the destination pool with extents preserved and orphans as spaces; same-pool ids copy raw like the reference; foreign ids can never land
  - closed: `drawText`/`drawGrapheme` ported as buffer-level primitives; `drawTextBuffer`, `drawImage`, and renderer-driven vectors stay with text-view, media-image, and render
  - closed: placement compositing copies clipped geometry with id remapping while pixels stay source-owned; the `drawImage` merge vectors stay with media-image
  - closed: SPEC-022 faults precise per requirement, including the finding that a slow-path-only fault passes everything (fast path has independent coverage); all faults reverted in-session

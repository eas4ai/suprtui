# Review: buffer-core

commit: 21d27aa3487fda663166856e077e7883901ed4f8
findings:
  - closed: color model, palette, cells, bounds, resize, and clear land with 7 spec-literal unit tests plus 18 buffer and 11 link ported vectors; full workspace green, clippy zero warnings, fmt clean, no `unsafe`
  - closed: link pool and tracker ported in full (including generation retirement and interning) as the buffer's caller-owned dependency; global pools excluded by the UNI-008 rule
  - closed: text-drawing cases ported as explicit `set` loops with the lowering documented; `drawText` itself stays with the drawing commitment
  - closed: image placements keep geometry plus handle while decoded pixels stay with media; `clear` drops placement geometry, covering the BUF-007 image clause at the geometry level
  - closed: SPEC-022 faults precise per requirement; two coarser faults showed BUF-003/BUF-007 share the link and clear primitives, and the precise replacements are recorded in the commitment

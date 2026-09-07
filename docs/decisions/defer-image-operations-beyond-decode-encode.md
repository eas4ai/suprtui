# Decision: defer image operations beyond decode/encode

Status: Decided 2026-09-07.

The `image` crate covers decode (PNG/JPEG/GIF/WebP) and PNG
re-encoding (MED-001/002, verified). The reference `image.zig`
additionally does resize, transform, extract, extend, blend, ICC
color management, and probe/inspect. None of the ported
commitments needs them: the renderer materializes fallbacks from
decoded RGBA, and no caller resizes, recolors, or composites.

Rationale: each operation is a correctness surface of its own
(ICC profiles especially), and pulling a second engine crate for
unused paths adds dependency risk for zero committed behavior.
Revisit when a caller needs resize or color management; the seam
is `DecodedImage` in `src/media.rs`.

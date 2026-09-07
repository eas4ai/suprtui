# Defer image operations beyond decode/encode

Level: Consequential
Decided by: Shawn
Rests on: docs/spec/media.md:MED-001,docs/spec/media.md:MED-002
Would be wrong if: A committed caller needs resize, transform, extract, ICC color management, or probe/inspect

## Decision

The `image` crate covers decode (PNG/JPEG/GIF/WebP) and PNG re-encoding (MED-001/002, verified). The reference `image.zig` additionally does resize, transform, extract, extend, blend, ICC color management, and probe/inspect. None of the ported commitments needs them: the renderer materializes fallbacks from decoded RGBA, and no caller resizes, recolors, or composites. Each deferred operation is a correctness surface of its own (ICC profiles especially), so they stay out until a caller needs them; the seam is `DecodedImage` in `src/media.rs`.

## Realized by

- ce9e311 Declare text-gaps scope (TXT-011..015) with deferral decisions

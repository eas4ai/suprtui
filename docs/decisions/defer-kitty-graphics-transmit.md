# Defer kitty graphics transmit state machine

Level: Consequential
Decided by: Shawn
Rests on: docs/spec/term.md:TRM-010,docs/spec/render.md:REN-010
Would be wrong if: An image-placing caller needs server-side pixels instead of fallback art

## Decision

The reference `kitty-transport.zig` runs leases, probes, compression fallback, timeouts, and chunked transmit. The port detects Kitty support and resolves the image protocol (TRM-010), and the renderer falls back to quadrant materialization without support (REN-010) — but nothing transmits pixels, so Kitty-capable terminals currently get fallback art. Transmit is a protocol session whose failure modes no committed vector exercises; shipping it untested would be worse than the honest fallback. Revisit when an image-placing caller lands; the seam is `ImagePlacement` staging and `kitty_supported` in `src/render.rs`.

## Realized by

- ce9e311 Declare text-gaps scope (TXT-011..015) with deferral decisions

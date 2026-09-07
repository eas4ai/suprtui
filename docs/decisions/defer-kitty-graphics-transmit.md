# Decision: defer kitty graphics transmit state machine

Status: Decided 2026-09-07.

The reference `kitty-transport.zig` runs leases, probes,
compression fallback, timeouts, and chunked transmit over the
terminal. The port detects Kitty support and resolves the image
protocol (TRM-010), and the renderer falls back to quadrant
materialization without support (REN-010) — but nothing transmits
pixels, so Kitty-capable terminals currently get the fallback art.

Rationale: transmit is a protocol session with failure modes
(leases, timeouts, compression negotiation) that no committed
vector exercises, and shipping it untested would be worse than the
honest fallback. Revisit when an image-placing caller lands; the
seam is `ImagePlacement` staging in `src/render.rs` and
`kitty_supported`.

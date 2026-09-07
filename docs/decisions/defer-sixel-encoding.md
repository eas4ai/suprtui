# Decision: defer sixel encoding

Status: Decided 2026-09-07.

The reference `terminal-image.zig` quantizes RGBA to palettes and
encodes sixel payloads (framed/plain, tmux-wrapped). The port
resolves the sixel protocol (TRM-010, including forced-sixel
refusal) but never encodes a sixel sequence; sixel terminals get
the block fallback like everything without Kitty support.

Rationale: quantization is a quality surface (palette choice,
dithering) with 29 reference tests behind it, and the same
no-caller argument as kitty transmit applies. Revisit together
with kitty transmit when an image-placing caller lands; the seam
is `ResolvedProtocol::Sixel` in `src/term.rs`.

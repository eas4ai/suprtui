# Defer sixel encoding

Level: Consequential
Decided by: Shawn
Rests on: docs/spec/term.md:TRM-010
Would be wrong if: A sixel terminal needs encoded payloads instead of block fallback

## Decision

The reference `terminal-image.zig` quantizes RGBA to palettes and encodes sixel payloads (framed/plain, tmux-wrapped). The port resolves the sixel protocol (TRM-010, including forced-sixel refusal) but never encodes a sixel sequence; sixel terminals get the block fallback. Quantization is a quality surface (palette choice, dithering) with 29 reference tests behind it, and the same no-caller argument as kitty transmit applies. Revisit together with kitty transmit when an image-placing caller lands; the seam is `ResolvedProtocol::Sixel` in `src/term.rs`.

## Realized by

- ce9e311 Declare text-gaps scope (TXT-011..015) with deferral decisions

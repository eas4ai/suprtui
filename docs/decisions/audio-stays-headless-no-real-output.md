# Audio stays headless (no real output backend)

Level: Consequential
Decided by: Shawn
Rests on: docs/spec/media.md:MED-003,docs/spec/media.md:MED-004,docs/spec/media.md:MED-005,docs/spec/media.md:MED-006,docs/spec/media.md:MED-007
Would be wrong if: A consuming application needs audible sound

## Decision

The reference `audio.zig` drives miniaudio: real playback and capture streams on real devices. The port implements the full engine/device/stream/voice/group API against a deterministic test backend (MED-003…007, verified); sound never reaches hardware. Real audio needs a platform audio crate plus device-permission and latency behavior that CI cannot verify, while every committed consumer is a lifecycle test. Shipping an untestable output path would trade a verified headless engine for an unverified loud one. Revisit when a consuming application needs sound; the seam is `TestBackend` in `src/audio.rs`.

## Realized by

- ce9e311 Declare text-gaps scope (TXT-011..015) with deferral decisions

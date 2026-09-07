# Decision: audio stays headless (no real output backend)

Status: Decided 2026-09-07.

The reference `audio.zig` drives miniaudio: real playback and
capture streams on real devices. The port implements the full
engine/device/stream/voice/group API against a deterministic test
backend (MED-003…007, verified); sound never reaches hardware.

Rationale: real audio needs a platform audio crate (cpal/rodio)
plus device-permission and latency behavior that CI cannot verify,
while every committed consumer is a lifecycle test. Shipping an
untestable output path would trade a verified headless engine for
an unverified loud one. Revisit when a consuming application
needs sound; the seam is `TestBackend` in `src/audio.rs`.

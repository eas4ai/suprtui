# Media (MED)

Status: Agreed 2026-09-07.
Prefix: MED.

Reference: `reference/opentui-0.5.11/packages/native/src/image.zig`,
`reference/opentui-0.5.11/packages/native/src/audio.zig`, and the
fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/image_test.zig`
and `audio_test.zig`. The crate exposes this domain as `src/media.rs`.
Decoding and audio I/O arrive from Rust crates, decided with decision
records; this domain specifies observable behavior, not their source.

[MED-001]
Image decoding MUST accept PNG, JPEG, GIF, and WebP bytes and MUST
report dimensions with RGBA pixels, while corrupt input MUST return
an error and MUST never panic.
Falsifier: a fixture image decodes to wrong pixels, or corrupt bytes
panic.
Mechanism: `cargo test -p suprtui media::decode` over ported image
fixtures, including corrupt inputs.

[MED-002]
Cloned images MUST observe identical pixels to their source, and
PNG re-encoding MUST roundtrip dimensions and pixels.
Falsifier: a re-encoded image decodes to different pixels than the
original.
Mechanism: `cargo test -p suprtui media::image_roundtrip`.

[MED-003]
Every fallible audio operation MUST return a `Result`, and stopping
an unstarted engine or mixer MUST be safe.
Falsifier: any audio call panics, or stop-before-start reports
success it did not perform.
Mechanism: `cargo test -p suprtui media::engine_lifecycle`.

[MED-004]
Audio streams MUST account every written sample in their statistics,
MUST apply volume, pan, and group settings, and MUST reject writes
after close.
Falsifier: statistics omit written samples, or a closed stream
accepts a write.
Mechanism: `cargo test -p suprtui media::streams` with a test audio
backend.

[MED-005]
Device listing MUST report each device's name and default flag, and
selecting an out-of-range index MUST return an error.
Falsifier: an out-of-range select succeeds, or two listings of an
unchanged system disagree.
Mechanism: `cargo test -p suprtui media::devices` with a test audio
backend.

[MED-006]
Capture MUST deliver only frames actually recorded, with statistics
matching what reading consumed.
Falsifier: a read returns more frames than were recorded.
Mechanism: `cargo test -p suprtui media::capture` with a test audio
backend.

[MED-007]
Loaded sounds MUST be addressed by id, and using or unloading an
unknown id MUST return an error.
Falsifier: audio plays after its sound was unloaded, or an unknown
id unloads without error.
Mechanism: `cargo test -p suprtui media::sound_ids`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review confirmed MED-004 through
MED-006 name a test audio backend as their mechanism, because real
hardware is absent in CI and behavior must stay observable. It kept
MED-001's panic ban alongside corrupt-input fixtures, since decoders
are the most likely panic source. It recorded that miniaudio and the
C image shims are not ported; Rust replacements are decided-later
crate choices. No spec lint binary exists in this repository yet, so
the one-obligation rule was checked by hand.

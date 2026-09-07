# Image decode and PNG encode use the image crate

Level: Consequential
Decided by: Muse Code
Rests on: docs/spec/media.md:MED-001,docs/spec/media.md:MED-002
Would be wrong if: Ported MED vectors fail against the image crate, or its decoders panic on corrupt input

## Decision

MED-001 needs PNG, JPEG, GIF, and WebP decoding to RGBA with dimensions; MED-002 needs PNG re-encoding roundtrips. The image crate covers all four decoders plus PNG encoding in pure Rust with no build scripts or native code, so the no-unsafe keystone and offline build stay intact. Lossy fixtures assert dimensions, opacity, and near-canonical pixels rather than bit-exact values, since exact lossy output is decoder-specific.

## Realized by

- 2e3af29  Implement media-image MED-001/002 with image crate and ported fixtures

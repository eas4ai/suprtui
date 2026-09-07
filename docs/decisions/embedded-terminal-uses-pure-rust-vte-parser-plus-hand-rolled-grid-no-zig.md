# Embedded terminal uses pure-Rust vte parser plus hand-rolled grid, no Zig

Level: Consequential
Decided by: Muse Code
Rests on: docs/spec/term.md:TRM-006,docs/spec/term.md:TRM-007
Would be wrong if: Ported TRM vectors fail against the vte-based grid, or fixture coverage needs engine behavior vte cannot provide

## Decision

TRM-006 is engine-neutral: fixtures observe composed grids, which any conforming virtual-terminal engine produces. A pure-Rust vte parser plus a caller-owned screen grid satisfies both requirements with no build-time Zig dependency, so TRM-007 containment holds trivially and the no-unsafe keystone is intact. No Ghostty source, no prebuilt library, no pkg-config.

## Realized by

727b388 Implement term-embedded TRM-006/007 with vte engine and ported fixtures

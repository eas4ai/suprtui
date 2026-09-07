# suprtui — overview (keystone)

Status: Agreed 2026-09-07.

## What the software is

suprtui is a Rust library crate that ports the OpenTUI native engine
from Zig to Rust. The reference implementation is the read-only copy at
`reference/opentui-0.5.11/packages/native/src` (about 35 modules,
33,500 lines of Zig, plus C shims for images, audio, and clipboard).
The port covers the entire native package, module by module, as safe
Rust with a plain Rust API. Benchmarks (`src/bench/`) and example
programs (`examples/`) are out of scope and are not ported.

The engine draws user interfaces inside a terminal. It keeps a grid of
styled character cells, compares each new frame with the previous one,
and writes only the changed cells to the terminal. It also owns the
pieces a terminal UI needs around the drawing core: text storage and
editing, terminal input and state, flexbox layout, sound, images, and
clipboard access.

## The problem it solves

The reference engine only builds with the Zig toolchain plus vendored C
sources, and its callers reach it through a C interface. A Rust program
that wants this engine must therefore adopt two foreign toolchains and
an unsafe boundary. suprtui removes both: one `cargo add suprtui` gives
a memory-safe, cargo-native engine whose failures arrive as Rust
`Result` values, not crashes across a language boundary.

## What the software is not

- It is not a drop-in replacement for the Zig shared library. It
  exposes no C interface and keeps no function-name compatibility with
  `lib.zig`. TypeScript callers of `@opentui/core` cannot load it
  unchanged.
- It is not the widget or framework layer. Buttons, selects, inputs,
  and the React/Solid bindings live in the TypeScript packages and stay
  there. suprtui is the engine underneath them.
- It is not a fork that tracks upstream. The Zig copy under
  `reference/` is a fixed snapshot used only as a behavioral reference.
  Where the reference and safe Rust disagree, the specification wins.
- It is not a pixel-graphics library. All drawing targets character
  cells of a terminal; images arrive through terminal graphics
  protocols (Kitty, Sixel) subject to what the terminal supports.
- It carries no benchmarks and no example programs. The Zig benches
  and the `hello` example stay behind; performance is checked through
  the crate's own test suite instead.

## Components

The crate is organized in eight modules. Each module owns its state and
exposes it through Rust types; modules talk to each other through
function calls, never through shared mutable globals.

- `render` owns frame comparison and terminal output. It reads a
  finished cell grid and writes the smallest correct update stream. It
  owns the output backends (direct, buffered, feed) and the placement
  of images on the grid.
- `buffer` owns the cell grid: characters, colors, and text attributes
  per cell, plus the drawing operations (boxes, text runs, blending).
  It owns ANSI encoding of styled runs.
- `text` owns text storage and editing: the rope, text and edit
  buffers, viewport views over them, iterators, segments, and syntax
  styling data.
- `term` owns terminal state and input: the terminal state machine,
  key and mouse encoding, and the embedded virtual terminal used to
  host child programs.
- `uni` owns Unicode handling: UTF-8 decoding, grapheme segmentation,
  and display-width calculation. Every other module asks `uni` how wide
  a character is; none of them computes widths itself.
- `layout` owns box layout: flexbox measurement and arrangement of
  renderable nodes, and the native renderable objects that bind measured
  content to layout nodes.
- `media` owns sound and image decoding: audio playback and capture,
  and decoding of PNG, JPEG, GIF, and WebP bytes into RGBA bitmaps.
- `sys` owns platform and housekeeping services: opaque handle tables,
  the event bus, clipboard access (X11, Wayland, macOS, Windows),
  logging, the span feed, hyperlinks, and scrollback splitting.

Typical flow for one frame: a caller edits `text` buffers, arranges
boxes with `layout`, paints cells into a `buffer` grid, and hands the
grid to `render`, which emits the terminal update. `term` feeds keyboard
and mouse events back in the other direction.

## Technology choices

- Rust, as a Cargo library crate named `suprtui` at the repository
  root. Reason: the goal is a cargo-native dependency, and a library
  (not a binary) is what callers add. A choice at Judged level will be
  recorded with `cairn decide` once the tooling is present.
- Safe Rust first; `unsafe` only at operating-system and
  protocol boundaries, each use documented at the site. Reason: the
  port's value over the Zig original is safety with equal behavior.
- Outside pieces (flexbox layout, audio I/O, image codecs) come from
  existing Rust crates, not from ported C sources. Reason: the
  developer chose maintained Rust crates over carrying C code and its
  build complexity into the crate. Exact crate picks are decided per
  domain with decision records.
- The Zig sources and their tests are the behavioral oracle during the
  port, read but never modified. Reason: `reference/` is gitignored and
  fixed, so it cannot drift under the specification.
- Rust edition 2024, stable toolchain. Reason: a new crate takes the
  current edition; no existing code constrains it.

## Cross-cutting constraints

- Error handling: every fallible operation returns `Result` with a
  typed error; no panics on malformed input (owned by `sys`, error
  rules specified per domain).
- No process-global mutable state in library code (owned by `sys`).
- Unsafe code is confined, documented, and reviewed (owned by `sys`).
- Unicode correctness follows `uni` widths everywhere; no module keeps
  a second width table (owned by `uni`).
- Verification: `cargo test` for unit and integration tests and `cargo
  clippy` run before each commitment closes (owned by `sys`).

## Spec map

- `docs/spec/glossary.md` — project vocabulary, no prefix.
- `docs/spec/render.md` — Prefix: REN.
- `docs/spec/buffer.md` — Prefix: BUF.
- `docs/spec/text.md` — Prefix: TXT.
- `docs/spec/term.md` — Prefix: TRM.
- `docs/spec/uni.md` — Prefix: UNI.
- `docs/spec/layout.md` — Prefix: LAY.
- `docs/spec/media.md` — Prefix: MED.
- `docs/spec/sys.md` — Prefix: SYS.
- `docs/spec/roadmap.md` — ordered commitments, no prefix.

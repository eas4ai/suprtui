# Commitment: uni-pool

Status: Agreed 2026-09-07.

Requirements: UNI-008

## What it delivers

Caller-owned grapheme-cluster storage on top of the uni-width and
uni-segments layers: a slab pool with size classes, page growth,
generations, live-id interning, owned and unowned allocation, and a
per-buffer reference tracker. The reference process-global pool is
excluded by design: UNI-008 forbids process-global mutable state, so
every pool is an explicit caller-owned value.

## Records

- Specification: `docs/spec/uni.md` (UNI-008).
- Code: `src/uni/pool.rs` (wired as `uni::pool` in `src/uni/mod.rs`).
- Mechanism: `.cairn/mechanisms/uni-pool.md` with runner
  `scripts/uni-pool.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Ported vectors live
in `tests/uni_pool.rs`, one `#[test]` per ported case named
`req_008_<slug>` plus hand-ported cleanup and panic tests. The
UNI-008 falsifier additionally lives at the spec-literal unit path
`uni::pool_isolation` in `src/uni/mod.rs`. No new external
dependencies.

## Design deviations (both safety-preserving)

- Unowned slots store a shared borrow (`&'a [u8]`) instead of a raw
  pointer, so `get` still aliases caller memory (pointer-identity
  holds verbatim in the ported tests) with the lifetime checked by
  the compiler. The pool is therefore `GraphemePool<'a>`, and `get`
  borrows `&self`.
- `GraphemeTracker` shares its pool through `Rc<RefCell<..>>` so any
  number of trackers can reference one pool at the same time, exactly
  as the reference does with raw pointers.

## Tests that prove it

- `cargo test -p suprtui uni::pool_isolation` — passes exactly one test.
- `cargo test -p suprtui --test uni_pool` — all 66 pass (61 ported
  vectors plus isolation, cleanup, panic, and packing tests).
- `sh scripts/uni-pool.sh` — prints `cairn: UNI-008: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

UNI-008 passes its mechanism with the full ported suite, the
spec-literal isolation test passes, and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with a controlled fault: disabling the
generation check fails exactly UNI-008 (stale-id vectors go red)
while the uni-width and uni-segments suites stay green. The fault was
a source edit, reverted in the same session, not a setup error.

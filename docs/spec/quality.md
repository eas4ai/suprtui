# Standing spec: quality

Status: Proposed 2026-09-07.

Scope: every commitment in `docs/spec/roadmap.md` (uni-width,
uni-segments, uni-pool, buffer-core, buffer-draw, render-core,
render-terminal, text-store, text-view, term-core, term-embedded,
layout-engine, media-image, media-audio, sys-core, sys-clipboard),
open and closed.

## Lint requirement

- LINT-001 — `cargo clippy -p suprtui --all-targets` reports no
  warnings and `cargo fmt -p suprtui -- --check` is clean over the
  whole crate. Fix the code; never silence a lint without a
  recorded reason.

## Mechanism

- Mechanism: `.cairn/mechanisms/lint.md` with runner
  `scripts/lint.sh`, printing `cairn: LINT-001: pass` on success
  and `cairn: LINT-001: fail` otherwise.
- Inputs: `src/` and `tests/`. These inputs join every
  commitment's footprint: behavior-identical maintenance inside
  them (warning fixes, formatting, fixture hygiene that keeps the
  suite building from a fresh clone) is declared lint work, not
  scope drift, and is committed as its own record.

## Why

Three LOOP-035 stops in one day, all three for cross-cutting
maintenance outside the active footprint: a behavior-identical
clippy fix, a fixture that only builds because `reference/` is
present but untracked, and clippy's own `type_complexity` notes on
new code. Making clippy a real requirement with evidence ends
this escalation class.

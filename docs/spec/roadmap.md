# suprtui — roadmap

Status: Agreed 2026-09-07.

Current: sys-small

Ordered commitments, each a unit of scope named for its goal:

1. `uni-width` — crate scaffold plus Unicode width methods and
   tables (UNI-001, UNI-002, UNI-003).
2. `uni-segments` — grapheme segmentation, wrap and position search,
   line breaks, invalid-input handling, char packing (UNI-004,
   UNI-005, UNI-006, UNI-007, UNI-009).
3. `uni-pool` — caller-owned grapheme pool with isolation (UNI-008).
4. `buffer-core` — color model, cells, bounds, resize, clear
   (BUF-001 … BUF-007).
5. `buffer-draw` — blending, scissor, wide cells, ANSI output,
   compositing, cross-pool ids (BUF-008 … BUF-013).
6. `render-core` — frame publish, skip, cursor, backends, stats
   (REN-001 … REN-005, REN-011).
7. `render-terminal` — threading parity, lifecycle, hit grid,
   offsets, image fallback (REN-006 … REN-010).
8. `text-store` — storage edits, span tracking, undo, view dirt
   (TXT-001 … TXT-004).
9. `text-view` — wrapping, selection, cursor edits, highlights,
   styles, bounds (TXT-005 … TXT-010).
10. `term-core` — capabilities, modes, alt screen, key and mouse
    encoding, environment, clipboard budget, image protocol
    (TRM-001 … TRM-005, TRM-008, TRM-009, TRM-010).
11. `term-embedded` — embedded terminal on `libghostty-rs` with
    Zig-containment, responses (TRM-006, TRM-007).
12. `layout-engine` — flexbox behavior with the decided Rust engine
    (LAY-001 … LAY-004).
13. `media-image` — image decoding and roundtrips with the decided
    Rust crates (MED-001, MED-002).
14. `media-audio` — engine, streams, devices, capture, sound ids
    with a test backend (MED-003 … MED-007).
15. `sys-core` — ownership isolation, events, links, span feed,
    scrollback, logging, error discipline (SYS-001, SYS-002,
    SYS-004 … SYS-008).
16. `sys-clipboard` — headless-testable clipboard lifecycle and
    platform backends (SYS-003).
17. `sys-clipboard-backends` — real platform clipboard backends
    over helper processes with headless-tested routing (SYS-009).
18. `render-stdout` — real stdout render backend with
    memory/stdout stream parity (REN-012).
19. `text-gaps` — gesture and viewport selection, iterators, wrap
    cache, editor-view as a unit (TXT-011 … TXT-015).
20. `sys-small` — multi-listener event emitter and file logger
    (SYS-010, SYS-011).

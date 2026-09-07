# Review: buffer-core

commit: 21e1015b970959a02a3bba36305a18cf96afd46c
findings:
  - closed: color model, palette, cells, bounds, resize, and clear land with 7 spec-literal unit tests plus 18 buffer and 11 link ported vectors; full workspace green (480 tests), clippy zero warnings, fmt clean, no `unsafe`
  - closed: link pool and tracker ported in full (including generation retirement and interning) as the buffer's caller-owned dependency; global pools excluded by the UNI-008 rule
  - closed: text-drawing cases ported as explicit `set` loops with the lowering documented; `drawText` itself stays with the drawing commitment
  - closed: image placements keep geometry plus handle while decoded pixels stay with media; `clear` drops placement geometry, covering the BUF-007 image clause at the geometry level
  - closed: SPEC-022 faults fail exactly their requirement, all nine of them (seven originals plus the two first-coarse ones). The link-bit drop and tracker-clear skip first failed BUF-003/BUF-007 jointly through test coupling; after isolating both tests, re-runs show exact precision. Verification for the isolation change is the full suite plus both precise fault re-runs (implementation behavior unchanged); fresh `cairn check` recording is blocked by LOOP-035 while Current is render-core
  - closed: isolation re-applied after render-core closed via merge 0346edf (vehicle b, scope answer loop-035); the tracker-clear skip and link-bit drop re-demos fail exactly BUF-007 and BUF-003 against the merged tree, so the precision claim reproduces again

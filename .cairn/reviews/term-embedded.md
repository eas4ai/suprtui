# Review: term-embedded

commit: da4c597131261625738392cfae611d9d865652a3
findings:
  - closed: vte-parser engine with hand-rolled grid, scrollback viewport, wrap-aware selection extraction, dirty-row compose with clipping, cursor report, and exactly-once response drain under a 1 MiB cap land with 8 ported vectors; full suite green, clippy zero warnings, fmt clean, no `unsafe`, one new dependency (`vte` parser only)
  - closed: three real implementation bugs found by integration: CSI positionals read only the first param group (CUP column always defaulted), interior gap cells dropped from extraction instead of contributing spaces, and gap cells skipped the selection swap (tails inherit lead colors through buffer continuation propagation, gaps need explicit paint)
  - closed: SPEC-022 faults fail exactly their requirement, both (green palette entry fails exactly TRM-006, DSR `0n` → `1n` fails exactly TRM-007); all faults reverted in-session
  - closed: scope kept clean: a `src/term/` directory restructure was flattened back to `src/term_embedded.rs` after LOOP-035/LOOP-044 showed the rename escaping the declared inputs; input-side encoders stay out per the commitment (no TRM requirement names them)

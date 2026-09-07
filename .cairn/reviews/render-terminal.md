# Review: render-terminal

commit: 704725e33e05967e27e6f0b60a00b0116ff58c07
findings:
  - closed: threaded backend parity, lifecycle sequences, hit grid with scissor, render offsets with footer surface, and image fallback land with 5 spec-literal unit tests plus 5 integration vectors; full suite green, clippy zero warnings, fmt clean, no `unsafe`
  - closed: hit staging alone starts no frame and skipped frames drop staged hits, matching the reference finishSkippedFrame; the hit tests force their frames and say so
  - closed: lifecycle ports renderer-owned bytes only; capability queries, feature enables, mouse and keyboard toggles, title reset, and kitty deletes stay with the term domain, and the shutdown sleep workaround is omitted as timing, not bytes
  - closed: SPEC-022 faults fail exactly their requirement, all five (dropped threaded bytes, kept alt screen, staged-grid hit read, ignored offset, cleared fallback); all faults reverted in-session

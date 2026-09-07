# BUF-003/007 test isolation restored out of render-core window

Surfaced from: BUF-003
Captured: 2026-09-07T17:15:47.853Z

21e1015 removed the release-on-clear assertion from req_003_link_per_cell_counting so the counting test owns no clear behavior (BUF-007 covers it). The edit landed inside render-core's window; per LOOP-035 it is restored to the activation tree here and preserved in history at 21e1015. Cost: buffer-core SPEC-022 demos recouple if re-run; their recorded precision stands.

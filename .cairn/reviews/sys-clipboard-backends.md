# Review: sys-clipboard-backends

commit: d936544861a354b776af2022a5f9526f3cea3a96
findings:
  - closed: Wayland, X11, macOS, and Windows helper routes build exact commands verified against the fake call log (wl-copy/wl-paste --no-newline, xclip selection clipboard with xsel fallback, pbcopy/pbpaste, clip plus powershell Get-Clipboard -Raw); 13 integration vectors plus 5 lib unit tests green, crate-wide clippy zero, fmt clean, no `unsafe`, no `static`, no new dependencies
  - closed: routing ports the reference truth table case-for-case — the 6-case select test carries the reference attempt counts, display vars must be non-empty, WSL markers are presence-based, kernel releases match the reference strings, and resolve_route falls back across displays with availability exactly as the reference selection outcome
  - closed: SPEC-022 faults discriminate exactly (swapped Wayland route fails only route_matrix, Failed-for-missing fails only missing_helper_is_unsupported, dropped xsel fallback fails only xclip_falls_back_to_xsel); all faults reverted in-session byte-clean
  - closed: a live write/read/clear roundtrip through auto-detect against this machine's X server passed (detect fell back Wayland-to-X11 as designed); recorded in the commitment doc, not the suite, which stays headless behind the CommandRunner seam
  - closed: raw-protocol X11/Wayland stays out deliberately — thousands of reference lines duplicating the helpers, and Wayland fd-passing is impossible in std-only safe Rust; the seam documents the boundary

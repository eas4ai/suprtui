# Review: render-stdout

commit: 675879041d5bc7913dd6330f1375d60bdb675a17
findings:
  - closed: committed bytes arrive in order through any `io::Write`; direct bytes flush immediately with sticky failure surfacing at `end_frame`; failed frames drop partials; empty frames write nothing; broken writers report `Failed` without panicking — 3 lib unit tests plus 3 integration vectors green
  - closed: the memory/stdout parity vector renders one multi-frame scene through both backends and demands byte-identical streams; Renderer gained only the additive `into_backend` accessor, no behavior change to existing paths
  - closed: SPEC-022 faults discriminate exactly (dropped direct bytes fail only order, swallowed writer errors fail only broken_writer); both reverted byte-clean
  - closed: full suite green, crate-wide clippy zero, fmt clean, no `unsafe`, no new dependencies

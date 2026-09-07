//! suprtui — Rust port of the OpenTUI native terminal engine.
//!
//! The specification in `docs/spec/` is the contract. Each module maps
//! to one domain: `uni` (UNI), `buffer` (BUF), `render` (REN), `text`
//! (TXT), `term` (TRM), `layout` (LAY), `media` (MED), `sys` (SYS).

pub mod ansi;
pub mod buffer;
pub mod link;
pub mod uni;

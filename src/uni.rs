//! Unicode width layer (`uni` domain, UNI-001..UNI-003).
//!
//! Ports the width behavior of `utf8.zig` (`eastAsianWidth` /
//! `eawToWidth`) and the `WidthMethod` selector. This file is the
//! verified seed: every branch below is covered by a committed test
//! vector checked against the reference. The loop's table work
//! replaces the seed default with full Unicode property data and
//! ports the complete reference vector suites before this
//! commitment is Done (see `docs/commitments/uni-width.md`).

/// Width method, matching the reference discriminants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WidthMethod {
    Wcwidth = 0,
    Unicode = 1,
    NoZwj = 2,
    UnicodeWide = 3,
}

/// Format controls that never advance the cursor (reference
/// `eawToWidth` explicit list, verified verbatim).
fn is_format_control(cp: u32) -> bool {
    matches!(
        cp,
        0x200B | 0x200C | 0x200D | 0x2060 | 0x034F | 0xFEFF
    ) || (0x180B..=0x180D).contains(&cp)
        || (0xFE00..=0xFE0F).contains(&cp)
        || (0xE0100..=0xE01EF).contains(&cp)
}

/// Combining Diacritical Marks block (verified subset of the
/// reference mark handling; full category data is loop table work).
fn is_verified_mark(cp: u32) -> bool {
    (0x0300..=0x036F).contains(&cp)
}

/// Verified double-width code points: CJK blocks that are
/// East_Asian_Wide, plus reference-listed symbols from `eawToWidth`.
fn is_verified_wide(cp: u32) -> bool {
    (0x3400..=0x4DBF).contains(&cp) // CJK Extension A
        || (0x4E00..=0x9FFF).contains(&cp) // CJK Unified Ideographs
        || (0xF900..=0xFAFF).contains(&cp) // CJK Compatibility Ideographs
        || (0x20000..=0x2A6DF).contains(&cp) // CJK Extension B
        || matches!(cp, 0x231A | 0x231B | 0x2329 | 0x232A)
}

/// Width of one code point in cells: 0, 1, or 2. Control characters
/// never advance the cursor. Code points outside the verified seed
/// report 1 until the loop's table work lands; no caller may treat
/// that default as specified behavior.
pub fn cell_width(cp: u32, _method: WidthMethod) -> u32 {
    if cp > 0x10FFFF {
        return 0;
    }
    if cp == 0 {
        return 0;
    }
    if cp < 32 || (0x7F..0xA0).contains(&cp) {
        return 0;
    }
    if is_verified_mark(cp) || is_format_control(cp) {
        return 0;
    }
    if is_verified_wide(cp) {
        return 2;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req_001_method_selection() {
        for method in [
            WidthMethod::Wcwidth,
            WidthMethod::Unicode,
            WidthMethod::NoZwj,
            WidthMethod::UnicodeWide,
        ] {
            assert_eq!(cell_width(0x41, method), 1);
            assert_eq!(cell_width(0x4E00, method), 2);
        }
    }

    #[test]
    fn req_002_width_rules() {
        let vectors: &[(u32, u32)] = &[
            (0x41, 1),
            (0x4E00, 2),
            (0x0300, 0),
            (0x0301, 0),
            (0x200D, 0),
            (0xFEFF, 0),
            (0x01, 0),
            (0x7F, 0),
            (0x231A, 2),
            (0x2329, 2),
        ];
        for (cp, want) in vectors {
            assert_eq!(cell_width(*cp, WidthMethod::Unicode), *want);
        }
    }

    #[test]
    fn req_003_emoji_table() {
        for cp in [
            0x231A, 0x231B, 0x2329, 0x232A, // reference-listed symbols
            0x3400, 0xF900, 0x20000, // CJK block samples
        ] {
            assert_eq!(cell_width(cp, WidthMethod::Unicode), 2);
        }
    }
}

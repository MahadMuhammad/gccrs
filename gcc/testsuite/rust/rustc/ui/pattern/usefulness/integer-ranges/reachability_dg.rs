#![allow(overlapping_range_endpoints)]
#![allow(non_contiguous_range_endpoints)]
#![deny(unreachable_patterns)]

macro_rules! m {
    ($s:expr, $t1:pat, $t2:pat) => {
        match $s {
            $t1 => {}
            $t2 => {}
            _ => {}
        }
    };
}

#[rustfmt::skip]
fn main() {
    m!(0u8, 42, 41);
    m!(0u8, 42, 42); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 42, 43);

    m!(0u8, 20..=30, 19);
    m!(0u8, 20..=30, 20); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 21); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 25); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 29); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 30); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 31);
    m!(0u8, 20..30, 19);
    m!(0u8, 20..30, 20); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..30, 21); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..30, 25); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..30, 29); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..30, 30);
    m!(0u8, 20..30, 31);

    m!(0u8, 20..=30, 20..=30); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20.. 30, 20.. 30); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 20.. 30); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 19..=30);
    m!(0u8, 20..=30, 21..=30); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 20..=29); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..=30, 20..=31);
    m!('a', 'A'..='z', 'a'..='z'); // { dg-error "" "" { target *-*-* } }

    match 0u8 {
        5 => {},
        6 => {},
        7 => {},
        8 => {},
        5..=8 => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    }
    match 0u8 {
        0..10 => {},
        10..20 => {},
        5..15 => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    }
    match 0u8 {
        0..10 => {},
        10..20 => {},
        20..30 => {},
        5..25 => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    }
    match 0u8 {
        0..10 => {},
        10 => {},
        11..=23 => {},
        19..30 => {},
        5..25 => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    }
    match 0usize {
        0..10 => {},
        10..20 => {},
        5..15 => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    }
    // Chars between '\u{D7FF}' and '\u{E000}' are invalid even though ranges that contain them are
    // allowed.
    match 'a' {
        _ => {},
        '\u{D7FF}'..='\u{E000}' => {}, // { dg-error "" "" { target *-*-* } }
    }
    match 'a' {
        '\u{0}'..='\u{D7FF}' => {},
        '\u{E000}'..='\u{10_FFFF}' => {},
        '\u{D7FF}'..='\u{E000}' => {}, // { dg-error "" "" { target *-*-* } }
    }

    match (0u8, true) {
        (0..=255, false) => {}
        (0..=255, true) => {} // ok
    }
    match (true, 0u8) {
        (false, 0..=255) => {}
        (true, 0..=255) => {} // ok
    }

    const FOO: i32 = 42;
    const BAR: &i32 = &42;
    match &0 {
        &42 => {}
        &FOO => {} // { dg-error "" "" { target *-*-* } }
        BAR => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    // Regression test, see https://github.com/rust-lang/rust/pull/66326#issuecomment-552889933
    match &0 {
        BAR => {} // ok
        _ => {}
    }
}


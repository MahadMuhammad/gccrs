#![deny(non_contiguous_range_endpoints)]

macro_rules! m {
    ($s:expr, $t1:pat, $t2:pat) => {
        match $s {
            $t1 => {}
            $t2 => {}
            _ => {}
        }
    };
}

fn main() {
    match 0u8 {
        20..30 => {} // { dg-error "" "" { target *-*-* } }
        31..=40 => {}
        _ => {}
    }
    match 0u8 {
        20..30 => {} // { dg-error "" "" { target *-*-* } }
        31 => {}
        _ => {}
    }

    m!(0u8, 20..30, 31..=40); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 31..=40, 20..30); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..30, 29..=40); // { dg-warning "" "" { target *-*-* } }
    m!(0u8, 20..30, 30..=40);
    m!(0u8, 20..30, 31..=40); // { dg-error "" "" { target *-*-* } }
    m!(0u8, 20..30, 32..=40);
    m!(0u8, 20..30, 31..=32); // { dg-error "" "" { target *-*-* } }
    // Don't lint two singletons.
    m!(0u8, 30, 32);
    // Don't lint on the equivalent inclusive range
    m!(0u8, 20..=29, 31..=40);
    // Don't lint if the lower one is a singleton.
    m!(0u8, 30, 32..=40);

    // Catch the `lo..MAX` case.
    match 0u8 {
        0..255 => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    // Except for `usize`, since `0..=usize::MAX` isn't exhaustive either.
    match 0usize {
        0..usize::MAX => {}
        _ => {}
    }

    // Don't lint if the gap is caught by another range.
    match 0u8 {
        0..10 => {}
        11..20 => {}
        10 => {}
        _ => {}
    }
    match 0u8 {
        0..10 => {}
        11..20 => {}
        5..15 => {}
        _ => {}
    }
    match 0u8 {
        0..255 => {}
        255 => {}
    }

    // Test multiple at once
    match 0u8 {
        0..10 => {} // { dg-error "" "" { target *-*-* } }
        11..20 => {}
        11..30 => {}
        _ => {}
    }
    match 0u8 {
        0..10 => {}  // { dg-error "" "" { target *-*-* } }
        11..20 => {} // { dg-error "" "" { target *-*-* } }
        21..30 => {}
        _ => {}
    }
    match 0u8 {
        00..20 => {} // { dg-error "" "" { target *-*-* } }
        10..20 => {} // { dg-error "" "" { target *-*-* } }
        21..30 => {}
        21..40 => {}
        _ => {}
    }

    // Test nested
    match (0u8, true) {
        (0..10, true) => {} // { dg-error "" "" { target *-*-* } }
        (11..20, true) => {}
        _ => {}
    }
    match (true, 0u8) {
        (true, 0..10) => {} // { dg-error "" "" { target *-*-* } }
        (true, 11..20) => {}
        _ => {}
    }
    // Asymmetry due to how exhaustiveness is computed.
    match (0u8, true) {
        (0..10, true) => {} // { dg-error "" "" { target *-*-* } }
        (11..20, false) => {}
        _ => {}
    }
    match (true, 0u8) {
        (true, 0..10) => {}
        (false, 11..20) => {}
        _ => {}
    }
    match Some(0u8) {
        Some(0..10) => {} // { dg-error "" "" { target *-*-* } }
        Some(11..20) => {}
        _ => {}
    }
}


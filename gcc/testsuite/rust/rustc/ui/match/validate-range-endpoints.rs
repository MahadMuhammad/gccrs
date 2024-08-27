#![feature(inline_const_pat)]
#![allow(overlapping_range_endpoints)]

fn main() {
    const TOO_BIG: u8 = 256;
    match 0u8 {
        1..257 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        1..=256 => {}
// { dg-error "" "" { target *-*-* } .-1 }

        // overflow is detected in a later pass for these
        0..257 => {}
        0..=256 => {}
        256..=100 => {}

        // There isn't really a way to detect these
        1..=TOO_BIG => {}
// { dg-error ".E0030." "" { target *-*-* } .-1 }
        1..=const { 256 } => {}
// { dg-error ".E0030." "" { target *-*-* } .-1 }
        _ => {}
    }

    match 0u64 {
        10000000000000000000..=99999999999999999999 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }

    match 0i8 {
        0..129 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        0..=128 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        -129..0 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        -10000..=-20 => {}
// { dg-error "" "" { target *-*-* } .-1 }

        // overflow is detected in a later pass for these
        128..=0 => {}
        0..-129 => {}
        -10000..=0 => {}
        _ => {}
    }

    // FIXME: error message is confusing
    match 0i8 {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        -10000..=0 => {}
    }
    match 0i8 {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        -10000.. => {}
    }
}


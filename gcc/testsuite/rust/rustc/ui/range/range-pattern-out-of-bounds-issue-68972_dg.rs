#![allow(unreachable_patterns)]
fn main() {
    match 0u8 {
        251..257 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        251..=256 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}


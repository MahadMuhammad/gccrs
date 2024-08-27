#![feature(strict_provenance)]
#![deny(fuzzy_provenance_casts)]

fn main() {
    let dangling = 16_usize as *const u8;
// { dg-error "" "" { target *-*-* } .-1 }
}


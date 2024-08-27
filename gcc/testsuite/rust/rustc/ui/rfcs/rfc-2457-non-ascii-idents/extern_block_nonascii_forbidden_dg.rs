#![feature(extern_types)]

extern "C" {
    type 一; // { dg-error "" "" { target *-*-* } }
    fn 二(); // { dg-error "" "" { target *-*-* } }
    static 三: usize; // { dg-error "" "" { target *-*-* } }
}

fn main() {}


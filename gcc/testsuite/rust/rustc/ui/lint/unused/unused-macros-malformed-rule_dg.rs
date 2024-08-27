#![deny(unused_macros)]

macro_rules! foo { // { dg-error "" "" { target *-*-* } }
    (v) => {};
    () => 0; // { dg-error "" "" { target *-*-* } }
}

macro_rules! bar {
    (v) => {};
    () => 0; // { dg-error "" "" { target *-*-* } }
}

fn main() {
    bar!(v);
}


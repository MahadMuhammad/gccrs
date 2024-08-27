#![deny(unused_macro_rules)]

macro_rules! foo {
    (v) => {};
    (w) => {};
    () => 0; // { dg-error "" "" { target *-*-* } }
}

fn main() {
    foo!(v);
}


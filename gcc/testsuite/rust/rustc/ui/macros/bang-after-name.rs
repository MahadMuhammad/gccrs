//@ run-rustfix
#[allow(unused_macros)]

macro_rules! foo! { // { dg-error "" "" { target *-*-* } }
    () => {};
}

fn main() {}


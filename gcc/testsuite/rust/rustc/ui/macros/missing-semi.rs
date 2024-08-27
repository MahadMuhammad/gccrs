#[allow(unused_macros)]
macro_rules! foo {
    () => {

    }
    () => {
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}


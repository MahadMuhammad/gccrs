//@ compile-flags: -Z trace-macros

#![recursion_limit = "5"]

fn main() {
    macro_rules! stack {
        ($overflow:expr) => {
            println!(stack!($overflow));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        };
    }

    stack!("overflow");
}


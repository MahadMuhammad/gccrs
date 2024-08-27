#![allow(incomplete_features, dead_code)]
#![deny(unconditional_recursion)] // { dg-note "" "" { target *-*-* } }
#![feature(explicit_tail_calls)]

fn f(x: bool) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    if x {
        become f(!x)
    } else {
        f(!x) // { dg-note "" "" { target *-*-* } }
    }
}

// This should *not* lint, tail-recursive functions which never return is a reasonable thing
fn g(x: bool) {
    if x {
        become g(!x)
    } else {
        become g(!x)
    }
}

fn main() {}


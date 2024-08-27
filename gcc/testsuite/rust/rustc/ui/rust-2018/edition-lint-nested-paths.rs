//@ run-rustfix

#![deny(absolute_paths_not_starting_with_crate)]

use foo::{a, b};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

mod foo {
    pub(crate) fn a() {}
    pub(crate) fn b() {}
    pub(crate) fn c() {}
}

fn main() {
    a();
    b();

    {
        use foo::{self as x, c};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
        x::a();
        c();
    }
}


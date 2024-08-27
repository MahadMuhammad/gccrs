//@ run-rustfix

#![deny(absolute_paths_not_starting_with_crate)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub(crate) mod foo {
    pub(crate) mod bar {
        pub(crate) mod baz { }
        pub(crate) mod baz1 { }

        pub(crate) struct XX;
    }
}

use foo::{bar::{baz::{}}};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

use foo::{bar::{XX, baz::{}}};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }

use foo::{bar::{baz::{}, baz1::{}}};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }

fn main() {
}


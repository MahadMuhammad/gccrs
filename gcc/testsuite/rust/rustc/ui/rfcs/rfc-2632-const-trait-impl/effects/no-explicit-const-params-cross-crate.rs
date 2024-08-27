//@ aux-build: cross-crate.rs
extern crate cross_crate;

use cross_crate::{Bar, foo};

fn main() {
    foo::<true>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    <() as Bar<true>>::bar();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

const FOO: () = {
    foo::<false>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    <() as Bar<false>>::bar();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
};


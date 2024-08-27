//@ aux-build: rustc_confusables_across_crate.rs

#![feature(rustc_attrs)]

extern crate rustc_confusables_across_crate;

use rustc_confusables_across_crate::BTreeSet;

fn main() {
    // Misspellings (similarly named methods) take precedence over `rustc_confusables`.
    let x = BTreeSet {};
    x.inser();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    x.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    x.push();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    x.test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    x.pulled();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

struct Bar;

impl Bar {
    #[rustc_confusables()]
// { dg-error "" "" { target *-*-* } .-1 }
    fn baz() {}

    #[rustc_confusables]
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    fn qux() {}

    #[rustc_confusables(invalid_meta_item)]
// { dg-error ".E0539." "" { target *-*-* } .-1 }
// { help ".E0539." "" { target *-*-* } .-2 }
    fn quux() {}
}

#[rustc_confusables("blah")]
// { dg-error "" "" { target *-*-* } .-1 }
fn not_inherent_impl_method() {}


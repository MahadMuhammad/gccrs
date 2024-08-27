//@ aux-build:unstable-macros.rs

#![feature(decl_macro)]
#![feature(staged_api)]
#![stable(feature = "rust1", since = "1.0.0")]

#[macro_use]
extern crate unstable_macros;

#[unstable(feature = "local_unstable", issue = "none")]
macro_rules! local_unstable { () => () }

#[unstable(feature = "local_unstable", issue = "none")]
macro local_unstable_modern() {}

#[stable(feature = "deprecated_macros", since = "1.0.0")]
#[deprecated(since = "1.0.0", note = "local deprecation note")]
#[macro_export]
macro_rules! local_deprecated{ () => () }

fn main() {
    local_unstable!(); // { dg-error ".E0658." "" { target *-*-* } }
    local_unstable_modern!(); // { dg-error ".E0658." "" { target *-*-* } }
    unstable_macro!(); // { dg-error ".E0658." "" { target *-*-* } }
    // unstable_macro_modern!(); // ERROR use of unstable library feature 'unstable_macros'

    deprecated_macro!();
// { dg-warning "" "" { target *-*-* } .-1 }
    local_deprecated!();
// { dg-warning "" "" { target *-*-* } .-1 }
}


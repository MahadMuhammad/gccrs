//@ compile-flags: --edition 2021
#![deny(unused_imports, redundant_imports)]
#![allow(dead_code)]

fn test0() {
    // Test remove FlatUnused
    use std::convert::TryFrom;
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = u32::try_from(5i32);
}

fn test1() {
    // FIXME(yukang) Test remove NestedFullUnused
    use std::convert::{TryFrom, TryInto};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    let _ = u32::try_from(5i32);
    let _a: i32 = u32::try_into(5u32).unwrap();
}

fn test2() {
    // FIXME(yukang): Test remove both redundant and unused
    use std::convert::{AsMut, Into};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    let _a: u32 = (5u8).into();
}

fn test3() {
    // Test remove NestedPartialUnused
    use std::convert::{From, Infallible};
// { dg-error "" "" { target *-*-* } .-1 }

    trait MyTrait {}
    impl MyTrait for fn() -> Infallible {}
}

fn main() {}


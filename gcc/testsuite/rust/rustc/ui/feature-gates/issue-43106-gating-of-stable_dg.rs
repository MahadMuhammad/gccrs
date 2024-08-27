// Testing gating of `#[stable]` in "weird" places.
//
// This file sits on its own because these signal errors, making
// this test incompatible with the "warnings only" nature of
// issue-43106-gating-of-builtin-attrs.rs

#![stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }

#[stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
mod stable {
    mod inner {
        #![stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    }

    #[stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    fn f() {}

    #[stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    struct S;

    #[stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    type T = S;

    #[stable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    impl S {}
}

fn main() {}


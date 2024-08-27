// Testing gating of `#[unstable]` in "weird" places.
//
// This file sits on its own because these signal errors, making
// this test incompatible with the "warnings only" nature of
// issue-43106-gating-of-builtin-attrs.rs

#![unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }

#[unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
mod unstable {
    mod inner {
        #![unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    }

    #[unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    fn f() {}

    #[unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    struct S;

    #[unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    type T = S;

    #[unstable()]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
    impl S {}
}

fn main() {}


#![feature(rustc_attrs)]
#![allow(private_interfaces)]

struct SemiPriv;

mod m {
    #[rustc_effective_visibility]
    struct Priv;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    #[rustc_effective_visibility]
    pub fn foo() {} // { dg-error "" "" { target *-*-* } }

    #[rustc_effective_visibility]
    impl crate::SemiPriv { // { dg-error "" "" { target *-*-* } }
        pub fn f(_: Priv) {}
    }
}

fn main() {}


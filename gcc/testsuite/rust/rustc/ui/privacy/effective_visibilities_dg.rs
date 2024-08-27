#![rustc_effective_visibility] // { dg-error "" "" { target *-*-* } }
#![feature(rustc_attrs)]

#[rustc_effective_visibility]
mod outer { // { dg-error "" "" { target *-*-* } }
    #[rustc_effective_visibility]
    pub mod inner1 { // { dg-error "" "" { target *-*-* } }

        #[rustc_effective_visibility]
        extern "C" {} // { dg-error "" "" { target *-*-* } }

        #[rustc_effective_visibility]
        pub trait PubTrait { // { dg-error "" "" { target *-*-* } }
            #[rustc_effective_visibility]
            const A: i32; // { dg-error "" "" { target *-*-* } }
            #[rustc_effective_visibility]
            type B; // { dg-error "" "" { target *-*-* } }
        }

        #[rustc_effective_visibility]
        struct PrivStruct; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

        #[rustc_effective_visibility]
        pub union PubUnion { // { dg-error "" "" { target *-*-* } }
            #[rustc_effective_visibility]
            a: u8, // { dg-error "" "" { target *-*-* } }
            #[rustc_effective_visibility]
            pub b: u8, // { dg-error "" "" { target *-*-* } }
        }

        #[rustc_effective_visibility]
        pub enum Enum { // { dg-error "" "" { target *-*-* } }
            #[rustc_effective_visibility]
            A( // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
                #[rustc_effective_visibility]
                PubUnion,  // { dg-error "" "" { target *-*-* } }
            ),
        }
    }

    #[rustc_effective_visibility]
    macro_rules! none_macro { // { dg-error "" "" { target *-*-* } }
        () => {};
    }

    #[macro_export]
    #[rustc_effective_visibility]
    macro_rules! public_macro { // { dg-error "" "" { target *-*-* } }
        () => {};
    }

    #[rustc_effective_visibility]
    pub struct ReachableStruct { // { dg-error "" "" { target *-*-* } }
        #[rustc_effective_visibility]
        pub a: u8, // { dg-error "" "" { target *-*-* } }
    }
}

#[rustc_effective_visibility]
pub use outer::inner1; // { dg-error "" "" { target *-*-* } }

pub fn foo() -> outer::ReachableStruct { outer::ReachableStruct {a: 0} }

mod half_public_import {
    #[rustc_effective_visibility]
    pub type HalfPublicImport = u8; // { dg-error "" "" { target *-*-* } }
    #[rustc_effective_visibility]
    #[allow(non_upper_case_globals)]
    pub(crate) const HalfPublicImport: u8 = 0; // { dg-error "" "" { target *-*-* } }
}

#[rustc_effective_visibility]
pub use half_public_import::HalfPublicImport; // { dg-error "" "" { target *-*-* } }

fn main() {}


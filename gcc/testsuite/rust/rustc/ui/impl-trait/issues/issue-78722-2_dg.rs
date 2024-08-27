//! test that we cannot register hidden types for opaque types
//! declared outside an anonymous constant.
// { dg-additional-options "-frust-edition=2018" }

#![feature(type_alias_impl_trait)]

type F = impl core::future::Future<Output = u8>;

struct Bug {
    V1: [(); {
        fn concrete_use() -> F {
// { dg-error ".E0271." "" { target *-*-* } .-1 }
            async {}
        }
        // FIXME(type_alias_impl_trait): inform the user about why `F` is not available here.
        let f: F = async { 1 };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        1
    }],
}

fn main() {}


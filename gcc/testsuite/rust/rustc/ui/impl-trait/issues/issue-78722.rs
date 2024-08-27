// { dg-additional-options "-frust-edition=2018" }

#![feature(type_alias_impl_trait)]

struct Bug {
    V1: [(); {
        type F = impl core::future::Future<Output = u8>;
        fn concrete_use() -> F {
// { dg-error ".E0271." "" { target *-*-* } .-1 }
            async {}
        }
        let f: F = async { 1 };
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        1
    }],
}

fn main() {}


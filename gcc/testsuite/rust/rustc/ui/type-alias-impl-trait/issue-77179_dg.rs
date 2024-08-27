// Regression test for the ICE in #77179.

#![feature(type_alias_impl_trait)]

type Pointer<T> = impl std::ops::Deref<Target = T>;

fn test() -> Pointer<_> {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
    Box::new(1)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {
    test();
}

extern "Rust" {
    fn bar() -> Pointer<_>;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}


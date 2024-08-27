#![feature(impl_trait_in_assoc_type)]

// We weren't checking that the trait and impl generics line up in the
// normalization-shortcut code in `OpaqueTypeCollector`.

use std::ops::Deref;

trait Foo {
    type Bar<'a>;

    type Baz<'a>;

    fn test<'a>() -> Self::Bar<'a>;
}

impl Foo for () {
    type Bar<'a> = impl Deref<Target = Self::Baz<'a>>;

    type Baz<T> = impl Sized;
// { dg-error ".E0049." "" { target *-*-* } .-1 }

    fn test<'a>() -> Self::Bar<'a> {
        &()
    }
}

fn main() {}


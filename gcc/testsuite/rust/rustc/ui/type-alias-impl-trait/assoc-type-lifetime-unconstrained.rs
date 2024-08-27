// Tests that we don't allow unconstrained lifetime parameters in impls when
// the lifetime is used in an associated opaque type.

#![feature(impl_trait_in_assoc_type)]

trait UnwrapItemsExt {
    type Iter;
    fn unwrap_items(self) -> Self::Iter;
}

struct MyStruct {}

trait MyTrait<'a> {}

impl<'a> MyTrait<'a> for MyStruct {}

impl<'a, I> UnwrapItemsExt for I {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type Iter = impl MyTrait<'a>;

    fn unwrap_items(self) -> Self::Iter {
        MyStruct {}
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    }
}

fn main() {}


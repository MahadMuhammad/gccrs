#![feature(impl_trait_in_assoc_type)]

pub trait A {
    type B;
    fn f(&self) -> Self::B;
}
impl<'a, 'b> A for () {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
// { dg-error ".E0207." "" { target *-*-* } .-2 }
    type B = impl core::fmt::Debug;

    fn f(&self) -> Self::B {}
// { dg-error ".E0792." "" { target *-*-* } .-1 }
}

fn main() {}


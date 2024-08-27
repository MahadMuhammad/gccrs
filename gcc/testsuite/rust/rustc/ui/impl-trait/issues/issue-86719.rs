#![feature(impl_trait_in_assoc_type)]

trait Bar {
    type E;
}
impl<S> Bar for S {
    type E = impl ; // { dg-error "" "" { target *-*-* } }
    fn foo() -> Self::E {
// { dg-error ".E0407." "" { target *-*-* } .-1 }
        |_| true // { dg-error ".E0282." "" { target *-*-* } }
    }
}
fn main() {}


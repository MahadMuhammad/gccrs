// { dg-additional-options "-frust-edition=2021" }
#![allow(bare_trait_objects)]
trait A: Sized {
    fn f(a: dyn A) -> dyn A;
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
}
trait B {
    fn f(a: dyn B) -> dyn B;
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
}
trait C {
    fn f(&self, a: dyn C) -> dyn C;
}

fn main() {}


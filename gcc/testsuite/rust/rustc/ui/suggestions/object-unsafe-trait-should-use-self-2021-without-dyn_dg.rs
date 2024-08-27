// { dg-additional-options "-frust-edition=2021" }
#![allow(bare_trait_objects)]
trait A: Sized {
    fn f(a: A) -> A;
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
// { dg-error ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }
}
trait B {
    fn f(b: B) -> B;
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
// { dg-error ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }
}
trait C {
    fn f(&self, c: C) -> C;
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
// { dg-error ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }
}

fn main() {}


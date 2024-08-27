//@ compile-flags: -Znext-solver

trait Foo {
    type Assoc;
}

trait Bar {}
fn needs_bar<S: Bar>() {}

fn test<T: Foo<Assoc = <T as Foo>::Assoc>>() {
    needs_bar::<T::Assoc>();
// { dg-error ".E0275." "" { target *-*-* } .-1 }
// { dg-error ".E0275." "" { target *-*-* } .-2 }
// { dg-error ".E0275." "" { target *-*-* } .-3 }
// { dg-error ".E0275." "" { target *-*-* } .-4 }
// { dg-error ".E0275." "" { target *-*-* } .-5 }
// { dg-error ".E0275." "" { target *-*-* } .-6 }
}

fn main() {}


//@ compile-flags: -Znext-solver

trait Foo1 {
    type Assoc1;
}

trait Foo2 {
    type Assoc2;
}

trait Bar {}
fn needs_bar<S: Bar>() {}

fn test<T: Foo1<Assoc1 = <T as Foo2>::Assoc2> + Foo2<Assoc2 = <T as Foo1>::Assoc1>>() {
    needs_bar::<T::Assoc1>();
// { dg-error ".E0275." "" { target *-*-* } .-1 }
// { dg-error ".E0275." "" { target *-*-* } .-2 }
// { dg-error ".E0275." "" { target *-*-* } .-3 }
// { dg-error ".E0275." "" { target *-*-* } .-4 }
// { dg-error ".E0275." "" { target *-*-* } .-5 }
// { dg-error ".E0275." "" { target *-*-* } .-6 }
}

fn main() {}


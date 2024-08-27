#![allow(warnings)]


trait A<T>
{
    fn get(&self) -> T { panic!() }
}

struct B<'a, T: 'a>(&'a (A<T> + 'a));

trait X { fn foo(&self) {} }

impl<'a, T> X for B<'a, T> {}

fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
    // oh dear!
    Box::new(B(&*v)) as Box<dyn X>
// { dg-error ".E0310." "" { target *-*-* } .-1 }
// { dg-error ".E0310." "" { target *-*-* } .-2 }
// { dg-error ".E0310." "" { target *-*-* } .-3 }
// { dg-error ".E0310." "" { target *-*-* } .-4 }
// { dg-error ".E0310." "" { target *-*-* } .-5 }
}

fn main() {}


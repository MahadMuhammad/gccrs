trait A<T> { }

struct B<'a, T:'a>(&'a (dyn A<T> + 'a));

trait X { }
impl<'a, T> X for B<'a, T> {}

fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'static> {
    Box::new(B(&*v)) as Box<dyn X>
// { dg-error ".E0310." "" { target *-*-* } .-1 }
// { dg-error ".E0310." "" { target *-*-* } .-2 }
// { dg-error ".E0310." "" { target *-*-* } .-3 }
// { dg-error ".E0310." "" { target *-*-* } .-4 }
// { dg-error ".E0310." "" { target *-*-* } .-5 }
// { dg-error ".E0310." "" { target *-*-* } .-6 }

}

fn main() {}


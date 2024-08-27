//@ compile-flags: -Znext-solver

trait Foo {}
trait Bar {}
trait Constrain {
    type Output;
}

impl<T, U> Foo for T
where
    T: Constrain<Output = U>,
    U: Bar,
{
}

impl Constrain for () {
    type Output = ();
}

fn needs_foo<T: Foo>() {}
fn main() {
    needs_foo::<()>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


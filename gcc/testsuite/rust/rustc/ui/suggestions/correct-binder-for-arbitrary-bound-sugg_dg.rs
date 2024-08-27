trait Foo
where
    for<'a> &'a Self: Bar,
{
}

impl Foo for () {}

trait Bar {}

impl Bar for &() {}

fn foo<T: Foo>() {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}


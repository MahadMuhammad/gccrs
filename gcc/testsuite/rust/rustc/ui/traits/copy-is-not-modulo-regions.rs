//@ revisions: not_static yes_static
//@[yes_static] check-pass

#[derive(Clone)]
struct Foo<'lt>(&'lt ());

impl Copy for Foo<'static> {}

#[derive(Clone)]
struct Bar<'lt>(Foo<'lt>);

#[cfg(not_static)]
impl<'any> Copy for Bar<'any> {}
// { dg-error "" "" { target *-*-* } .-1 }

#[cfg(yes_static)]
impl<'any> Copy for Bar<'static> {}

fn main() {}


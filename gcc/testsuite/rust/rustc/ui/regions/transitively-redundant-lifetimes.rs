#![deny(redundant_lifetimes)]

fn a<'a, 'b>(x: &'a &'b &'a ()) {} // { dg-error "" "" { target *-*-* } }

fn b<'a: 'b, 'b: 'a>() {} // { dg-error "" "" { target *-*-* } }

struct Foo<T: 'static>(T);
fn c<'a>(_: Foo<&'a ()>) {} // { dg-error "" "" { target *-*-* } }

struct Bar<'a>(&'a ());
impl<'a> Bar<'a> {
    fn d<'b: 'a>(&'b self) {} // { dg-error "" "" { target *-*-* } }
}

fn ok(x: &'static &()) {}

trait Tr<'a> {}
impl<'a: 'static> Tr<'a> for () {} // { dg-error "" "" { target *-*-* } }

fn main() {}


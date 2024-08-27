#![feature(associated_type_defaults)]

trait Foo {
    type A<'a> where Self: 'a;
    type B<'a, 'b> where 'a: 'b;
    type C where Self: Clone;
    fn d() where Self: Clone;
}

#[derive(Copy, Clone)]
struct Fooy<T>(T);

impl<T> Foo for Fooy<T> {
    type A<'a> = (&'a ()) where Self: 'static;
// { dg-error ".E0276." "" { target *-*-* } .-1 }
    type B<'a, 'b> = (&'a(), &'b ()) where 'b: 'a;
// { dg-error ".E0276." "" { target *-*-* } .-1 }
    type C = String where Self: Copy;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    fn d() where Self: Copy {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}


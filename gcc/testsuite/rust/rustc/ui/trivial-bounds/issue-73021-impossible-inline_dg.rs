//@ build-pass
//@ revisions: no-opt inline
//@ [inline]compile-flags: -Zmir-opt-level=3 --emit=mir
#![feature(trivial_bounds)]
#![allow(unused)]

trait Foo {
    fn test(&self);
}

fn foo<'a>(s: &'a mut ())
where
    &'a mut (): Foo,
{
    s.test();
}

fn clone(it: &mut ()) -> &mut ()
where
    for<'any> &'any mut (): Clone,
// { dg-warning "" "" { target *-*-* } .-1 }
{
    it.clone()
}

fn generic_function<X: Foo>(x: X) {}

struct S where i32: Foo;
// { dg-warning "" "" { target *-*-* } .-1 }

impl Foo for () where i32: Foo {
// { dg-warning "" "" { target *-*-* } .-1 }
    fn test(&self) {
        3i32.test();
        Foo::test(&4i32);
        generic_function(5i32);
    }
}

fn f() where i32: Foo {
// { dg-warning "" "" { target *-*-* } .-1 }
    let s = S;
    3i32.test();
    Foo::test(&4i32);
    generic_function(5i32);
}

fn g() where &'static str: Foo {
// { dg-warning "" "" { target *-*-* } .-1 }
    "Foo".test();
    Foo::test(&"Foo");
    generic_function("Foo");
}

fn use_op(s: String) -> String
where
    String: ::std::ops::Neg<Output = String>,
// { dg-warning "" "" { target *-*-* } .-1 }
{
    -s
}

fn use_for()
where
    i32: Iterator,
// { dg-warning "" "" { target *-*-* } .-1 }
{
    for _ in 2i32 {}
}

fn main() {}


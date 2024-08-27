//@ run-rustfix
#![allow(unused)]
fn a() => usize { 0 }
// { dg-error "" "" { target *-*-* } .-1 }

fn b(): usize { 0 }
// { dg-error "" "" { target *-*-* } .-1 }

fn bar(_: u32) {}

fn baz() -> *const dyn Fn(u32) { unimplemented!() }

fn foo() {
    match () {
        _ if baz() == &bar as &dyn Fn(u32) => (),
        () => (),
    }
}

fn main() {
    let foo = |a: bool| => bool { a };
// { dg-error "" "" { target *-*-* } .-1 }
    dbg!(foo(false));

    let bar = |a: bool|: bool { a };
// { dg-error "" "" { target *-*-* } .-1 }
    dbg!(bar(false));
}


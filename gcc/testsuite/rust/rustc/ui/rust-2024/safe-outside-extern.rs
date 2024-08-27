safe fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }

safe static FOO: i32 = 1;
// { dg-error "" "" { target *-*-* } .-1 }

trait Foo {
    safe fn foo();
// { dg-error "" "" { target *-*-* } .-1 }
}

impl Foo for () {
    safe fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
}

type FnPtr = safe fn(i32, i32) -> i32;
// { dg-error "" "" { target *-*-* } .-1 }

unsafe static LOL: u8 = 0;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


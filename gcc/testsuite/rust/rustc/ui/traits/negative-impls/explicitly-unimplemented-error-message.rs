// This tests issue #79683: note in the error message that the trait is
// explicitly unimplemented instead of suggesting to implement it.

#![feature(negative_impls)]

struct Qux;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

impl !Clone for Qux {}

trait Bar {
    fn bar(&self);
}

impl !Bar for u32 {}

trait Foo {
    fn foo(&self);
}
// { dg-note "" "" { target *-*-* } .-3 }

trait FooBar {
    fn foo(&self);
}

impl !Foo for Qux {}

impl !FooBar for Qux {}

impl !FooBar for u32 {}

fn main() {
    Qux.clone();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }
// { dg-note ".E0599." "" { target *-*-* } .-3 }

    0_u32.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }
// { dg-note ".E0599." "" { target *-*-* } .-3 }

    Qux.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }
// { dg-note ".E0599." "" { target *-*-* } .-3 }

    0_u32.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }
// { dg-note ".E0599." "" { target *-*-* } .-3 }
}


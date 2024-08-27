#![feature(unboxed_closures)]

extern "rust-call" fn b(_i: i32) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

trait Tr {
    extern "rust-call" fn a();
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    extern "rust-call" fn b() {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct Foo;

impl Foo {
    extern "rust-call" fn bar() {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

impl Tr for Foo {
    extern "rust-call" fn a() {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {
    b(10);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    Foo::bar();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    <Foo as Tr>::a();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    <Foo as Tr>::b();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


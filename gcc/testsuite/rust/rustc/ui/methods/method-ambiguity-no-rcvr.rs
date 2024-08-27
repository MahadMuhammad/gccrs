struct Qux;

trait Foo {
    fn foo();
}

trait FooBar {
    fn foo() {}
}

fn main() {
    Qux.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}


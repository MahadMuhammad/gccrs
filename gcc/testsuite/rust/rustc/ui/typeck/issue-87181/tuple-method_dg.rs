struct Bar<T> {
    bar: T
}

struct Foo(u8, i32);
impl Foo {
    fn foo() { }
}

fn main() {
    let thing = Bar { bar: Foo };
    thing.bar.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}


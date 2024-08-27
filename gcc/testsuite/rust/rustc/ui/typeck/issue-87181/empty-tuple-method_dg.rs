struct Bar<T> {
    bar: T
}

struct Foo();
impl Foo {
    fn foo(&self) { }
}

fn main() {
    let thing = Bar { bar: Foo };
    thing.bar.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}


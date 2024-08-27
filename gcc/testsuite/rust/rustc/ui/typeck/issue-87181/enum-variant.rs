struct Bar<T> {
    bar: T
}

enum Foo{
    Tup()
}
impl Foo {
    fn foo(&self) { }
}

fn main() {
    let thing = Bar { bar: Foo::Tup };
    thing.bar.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}


struct Bar<T> {
    bar: T
}

struct Foo(char, u16);
impl Foo {
    fn foo() { }
}

fn main() {
    let thing = Bar { bar: Foo };
    thing.bar.0;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
}


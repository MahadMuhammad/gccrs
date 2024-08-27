//@ run-rustfix

#[derive(Clone)]
struct Foo;
impl Foo {
    fn foo(self) {}
}
fn main() {
    let foo = &Foo;
    foo.foo(); // { dg-error ".E0507." "" { target *-*-* } }
}


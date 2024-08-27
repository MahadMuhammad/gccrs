//@ check-fail

struct Foo {}
impl Foo {
    fn bar(foo: Foo<Target = usize>) {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
}
fn main() {}


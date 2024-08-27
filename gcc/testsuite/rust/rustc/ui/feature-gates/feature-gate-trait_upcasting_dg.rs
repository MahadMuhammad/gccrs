trait Foo {}

trait Bar: Foo {}

impl Foo for () {}

impl Bar for () {}

fn main() {
    let bar: &dyn Bar = &();
    let foo: &dyn Foo = bar;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}


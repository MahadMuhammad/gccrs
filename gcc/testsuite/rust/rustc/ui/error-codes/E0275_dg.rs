//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"
trait Foo {}

struct Bar<T>(T);

impl<T> Foo for T where Bar<T>: Foo {} // { dg-error ".E0275." "" { target *-*-* } }

fn main() {
}


use std::ops::Add;

struct Wrapper<T>(T);

trait Foo {}

fn qux<T>(a: Wrapper<T>, b: T) -> T {
    a + b
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

fn main() {}


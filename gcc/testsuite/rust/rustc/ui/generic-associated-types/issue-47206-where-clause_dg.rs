// Check that this program doesn't cause the compiler to error without output.

trait Foo {
    type Assoc3<T>;
}

struct Bar;

impl Foo for Bar {
    type Assoc3<T> = Vec<T> where T: Iterator;
// { dg-error ".E0276." "" { target *-*-* } .-1 }
}

fn main() {}


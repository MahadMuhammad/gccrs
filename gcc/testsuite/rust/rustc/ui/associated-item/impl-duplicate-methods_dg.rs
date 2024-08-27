struct Foo;

impl Foo {
    fn orange(&self) {}
    fn orange(&self) {}
// { dg-error ".E0592." "" { target *-*-* } .-1 }
}

fn main() {}


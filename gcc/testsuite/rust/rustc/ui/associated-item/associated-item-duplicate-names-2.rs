struct Foo;

impl Foo {
    const bar: bool = true;
    fn bar() {} // { dg-error ".E0592." "" { target *-*-* } }
}

fn main() {}


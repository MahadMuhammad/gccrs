struct Foo {}

impl Foo {
    pub fn bar()
// { dg-error "" "" { target *-*-* } .-1 }
}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


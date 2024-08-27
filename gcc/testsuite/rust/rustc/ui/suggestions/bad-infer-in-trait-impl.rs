trait Foo {
    fn bar();
}

impl Foo for () {
    fn bar(s: _) {}
// { dg-error ".E0050." "" { target *-*-* } .-1 }
// { dg-error ".E0050." "" { target *-*-* } .-2 }
}

fn main() {}


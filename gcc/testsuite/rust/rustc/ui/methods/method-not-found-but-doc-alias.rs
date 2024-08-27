struct Foo;

impl Foo {
    #[doc(alias = "quux")]
    fn bar(&self) {}
}

fn main() {
    Foo.quux();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}


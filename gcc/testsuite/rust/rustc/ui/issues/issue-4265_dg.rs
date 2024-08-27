struct Foo {
      baz: usize
}

impl Foo {
    fn bar() {
        Foo { baz: 0 }.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    }

    fn bar() { // { dg-error ".E0592." "" { target *-*-* } }
    }
}

fn main() {}


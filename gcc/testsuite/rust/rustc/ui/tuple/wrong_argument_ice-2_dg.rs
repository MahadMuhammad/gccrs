fn test(t: (i32, i32)) {}

struct Foo;

impl Foo {
    fn qux(&self) -> i32 {
        0
    }
}

fn bar() {
    let x = Foo;
    test(x.qux(), x.qux());
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

fn main() {}


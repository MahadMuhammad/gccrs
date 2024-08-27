// Regression test of #86162.

fn gen<T>() -> T { todo!() }

struct Foo;

impl Foo {
    fn bar(x: impl Clone) {}
}

fn main() {
    Foo::bar(gen()); //<- Do not suggest `Foo::bar::<impl Clone>()`!
// { dg-error ".E0283." "" { target *-*-* } .-1 }
}


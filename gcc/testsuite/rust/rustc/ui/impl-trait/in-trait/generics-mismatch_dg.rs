struct U;

trait Foo {
    fn bar(&self) -> impl Sized;
}

impl Foo for U {
    fn bar<T>(&self) {}
// { dg-error ".E0049." "" { target *-*-* } .-1 }
}

fn main() {
    U.bar();
}


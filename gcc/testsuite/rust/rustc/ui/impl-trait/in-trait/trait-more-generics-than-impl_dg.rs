struct S;

trait Foo {
    fn bar<T>() -> impl Sized;
}

impl Foo for S {
    fn bar() -> impl Sized {}
// { dg-error ".E0049." "" { target *-*-* } .-1 }
}

fn main() {
    S::bar();
}


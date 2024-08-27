trait Foo {
    fn bar() -> impl std::fmt::Display;
}

impl Foo for () {
    fn bar() -> () {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}


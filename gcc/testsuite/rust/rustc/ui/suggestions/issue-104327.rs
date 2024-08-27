trait Bar {}

trait Foo {
    fn f() {}
}

impl Foo for dyn Bar {}

fn main() {
    Foo::f();
// { dg-error ".E0790." "" { target *-*-* } .-1 }
}


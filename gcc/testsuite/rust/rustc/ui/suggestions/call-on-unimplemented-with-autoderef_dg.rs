trait Foo {}

impl Foo for i32 {}

fn needs_foo(_: impl Foo) {}

fn test(x: &Box<dyn Fn() -> i32>) {
    needs_foo(x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
}

fn main() {}


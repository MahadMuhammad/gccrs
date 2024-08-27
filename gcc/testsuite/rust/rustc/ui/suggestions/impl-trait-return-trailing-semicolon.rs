trait Bar {}

impl Bar for i32 {}

struct Qux;

impl Bar for Qux {}

fn foo() -> impl Bar {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    5;
// { help "" "" { target *-*-* } .-1 }
}

fn bar() -> impl Bar {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
    "";
}

fn main() {}


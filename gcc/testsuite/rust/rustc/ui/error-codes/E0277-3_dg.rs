fn foo<T: PartialEq>(_: T) {}

struct S;

fn main() {
    foo(S);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


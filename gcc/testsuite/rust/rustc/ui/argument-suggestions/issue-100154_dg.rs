fn foo(i: impl std::fmt::Display) {}

fn main() {
    foo::<()>(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}


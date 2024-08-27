//@ check-pass
#[diagnostic::non_existing_attribute]
// { dg-warning "" "" { target *-*-* } .-1 }
pub trait Bar {
}

#[diagnostic::non_existing_attribute(with_option = "foo")]
// { dg-warning "" "" { target *-*-* } .-1 }
struct Foo;

fn main() {
}


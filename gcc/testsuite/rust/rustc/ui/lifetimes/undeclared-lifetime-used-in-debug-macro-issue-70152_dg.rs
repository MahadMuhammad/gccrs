#[derive(Eq, PartialEq)]
struct Test {
    a: &'b str,
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }
}

trait T {
    fn foo(&'static self) {}
}

impl T for Test {
    fn foo(&'b self) {} // { dg-error ".E0261." "" { target *-*-* } }
}

fn main() {}


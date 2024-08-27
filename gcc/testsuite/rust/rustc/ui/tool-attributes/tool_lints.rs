#[warn(foo::bar)]
// { dg-error ".E0710." "" { target *-*-* } .-1 }
// { dg-error ".E0710." "" { target *-*-* } .-2 }
fn main() {}


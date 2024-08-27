#[derive(Clone)] // { dg-error ".E0782." "" { target *-*-* } }
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
struct Foo;
trait Foo {} // { dg-error ".E0428." "" { target *-*-* } }
fn main() {}


#![deny(unknown_or_malformed_diagnostic_attributes)]
trait Foo {}

#[diagnostic::do_not_recommend]
// { dg-error "" "" { target *-*-* } .-1 }
impl Foo for i32 {}

fn main() {}


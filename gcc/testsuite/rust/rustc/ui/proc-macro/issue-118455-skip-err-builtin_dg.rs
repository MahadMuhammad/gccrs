#![some_nonexistent_attribute]
// { dg-error "" "" { target *-*-* } .-1 }
#[derive(Debug)]
pub struct SomeUserCode;

fn main() {}


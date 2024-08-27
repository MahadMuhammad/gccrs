#[repr(align(16))]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
pub type Foo = i32;

fn main() {}


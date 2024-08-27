trait Foo<const M: u8, const M: u8 = M> {}
// { dg-error ".E0403." "" { target *-*-* } .-1 }
impl Foo<2> for () {}
fn main() {}


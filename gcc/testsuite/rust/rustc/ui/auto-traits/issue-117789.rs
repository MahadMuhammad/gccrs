auto trait Trait<P> {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
impl<P> Trait<P> for () {}

fn main() {}


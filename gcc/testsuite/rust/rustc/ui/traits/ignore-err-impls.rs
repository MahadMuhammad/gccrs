pub struct S;

trait Generic<T> {}

impl<'a, T> Generic<&'a T> for S {}
impl Generic<Type> for S {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }

fn main() {}


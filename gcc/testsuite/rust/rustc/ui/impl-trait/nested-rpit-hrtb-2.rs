// The nested impl Trait references a higher-ranked region

trait Trait<'a> { type Assoc; }
impl<'a> Trait<'a> for () { type Assoc = &'a str; }

fn test() -> impl for<'a> Trait<'a, Assoc = impl Sized> {}
// { dg-error ".E0700." "" { target *-*-* } .-1 }

fn main() {}


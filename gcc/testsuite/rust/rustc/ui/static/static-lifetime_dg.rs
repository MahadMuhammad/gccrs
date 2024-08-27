pub trait Arbitrary: Sized + 'static {}

impl<'a, A: Clone> Arbitrary for ::std::borrow::Cow<'a, A> {} // { dg-error ".E0495." "" { target *-*-* } }
// { dg-error ".E0495." "" { target *-*-* } .-1 }

fn main() {
}


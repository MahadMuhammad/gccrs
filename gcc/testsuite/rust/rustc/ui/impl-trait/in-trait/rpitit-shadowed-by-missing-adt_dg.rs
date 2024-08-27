// issue: 113903

use std::ops::Deref;

pub trait Tr {
    fn w() -> impl Deref<Target = Missing<impl Sized>>;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

impl Tr for () {
    #[expect(refining_impl_trait)]
    fn w() -> &'static () {
        &()
    }
}

fn main() {}


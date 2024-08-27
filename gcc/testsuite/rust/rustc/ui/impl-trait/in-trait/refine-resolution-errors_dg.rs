// This is a non-regression test for issue #126670 where RPITIT refinement checking encountered
// errors during resolution and ICEd.

// { dg-additional-options "-frust-edition= 2018" }

pub trait Mirror {
    type Assoc;
}
impl<T: ?Sized> Mirror for () {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type Assoc = T;
}

pub trait First {
    async fn first() -> <() as Mirror>::Assoc;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
}

impl First for () {
    async fn first() {}
}

fn main() {}


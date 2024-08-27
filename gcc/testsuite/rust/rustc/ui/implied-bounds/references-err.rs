trait Identity {
    type Identity;
}
impl<T> Identity for T {
    type Identity = T;
}

trait Trait {
    type Assoc: Identity;
    fn tokenize(&self) -> <Self::Assoc as Identity>::Identity;
}

impl Trait for () {
    type Assoc = DoesNotExist;
// { dg-error ".E0412." "" { target *-*-* } .-1 }

    fn tokenize(&self) -> <Self::Assoc as Identity>::Identity {
        unimplemented!()
    }
}

fn main() {}


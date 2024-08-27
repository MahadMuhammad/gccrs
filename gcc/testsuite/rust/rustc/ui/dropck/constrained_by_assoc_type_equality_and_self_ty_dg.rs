trait Trait {
    type Assoc;
}

struct Foo<T: Trait, U: ?Sized>(T, U);

impl<T: Trait<Assoc = U>, U: ?Sized> Drop for Foo<T, U> {
// { dg-error ".E0367." "" { target *-*-* } .-1 }
    fn drop(&mut self) {}
}

fn main() {}


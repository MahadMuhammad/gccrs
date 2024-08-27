trait Id {
    type This: ?Sized;
}
impl<T: ?Sized> Id for T {
    type This = T;
}

trait Trait {
    type Assoc: Id<This: Copy>;
}

// We can't see use the `T::Assoc::This: Copy` bound to prove `T::Assoc: Copy`
fn foo<T: Trait>(x: T::Assoc) -> (T::Assoc, T::Assoc) {
    (x, x)
// { dg-error ".E0382." "" { target *-*-* } .-1 }
}

fn main() {}


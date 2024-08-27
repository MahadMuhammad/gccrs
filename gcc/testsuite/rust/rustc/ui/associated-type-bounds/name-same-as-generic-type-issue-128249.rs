trait Trait<Type> {
    type Type;

    fn one(&self, val:  impl Trait<Type: Default>);
// { dg-error ".E0107." "" { target *-*-* } .-1 }

    fn two<T: Trait<Type: Default>>(&self) -> T;
// { dg-error ".E0107." "" { target *-*-* } .-1 }

    fn three<T>(&self) -> T where
        T: Trait<Type: Default>,;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

fn main() {}


trait Trait {
    type Gat<U: ?Sized>;
}

fn test<T>(f: for<'a> fn(<&'a T as Trait>::Gat<&'a [str]>)) where for<'a> &'a T: Trait {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}


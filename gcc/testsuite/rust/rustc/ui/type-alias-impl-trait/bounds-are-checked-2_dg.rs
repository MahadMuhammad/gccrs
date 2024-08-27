// Make sure that we check that impl trait types implement the traits that they
// claim to.

#![feature(type_alias_impl_trait)]

mod foo {
    pub type X<T> = impl Clone;

    fn f<T: Clone>(t: T) -> X<T> {
        t
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }
}
use foo::X;

fn g<T>(o: Option<X<T>>) -> Option<X<T>> {
    o.clone()
}

fn main() {
    g(None::<X<&mut ()>>);
}


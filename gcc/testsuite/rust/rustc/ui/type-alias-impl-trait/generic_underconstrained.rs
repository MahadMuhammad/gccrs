#![feature(type_alias_impl_trait)]

fn main() {}

trait Trait {}
type Underconstrained<T: Trait> = impl Send;

// no `Trait` bound
fn underconstrain<T>(_: T) -> Underconstrained<T> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    unimplemented!()
}


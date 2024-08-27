#![feature(type_alias_impl_trait)]

//! This test used to ICE rust-lang/rust#124891
//! because we added an assertion for catching cases where opaque types get
//! registered during the processing of subtyping predicates.

type Tait = impl FnOnce() -> ();

fn reify_as_tait() -> Thunk<Tait> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    Thunk::new(|cont| cont)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

struct Thunk<F>(F);

impl<F> Thunk<F> {
    fn new(f: F)
    where
        F: ContFn,
    {
        todo!();
    }
}

trait ContFn {}

impl<F: FnOnce(Tait) -> ()> ContFn for F {}

fn main() {}


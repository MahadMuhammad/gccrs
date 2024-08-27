//! We evaluate `1 + 2` with `Reveal::All` during typeck, causing
//! us to get the concrete type of `Bar` while computing it.
//! This again requires type checking `foo`.
#![feature(type_alias_impl_trait)]
type Bar = impl Sized;
// { dg-error ".E0391." "" { target *-*-* } .-1 }

fn foo() -> Bar
where
    Bar: Send,
{
    [0; 1 + 2]
// { dg-error ".E0283." "" { target *-*-* } .-1 }
}

fn main() {}


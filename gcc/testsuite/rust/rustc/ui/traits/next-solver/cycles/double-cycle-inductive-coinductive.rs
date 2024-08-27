//@ compile-flags: -Znext-solver
#![feature(rustc_attrs)]

// Test that having both an inductive and a coinductive cycle
// is handled correctly.

#[rustc_coinductive]
trait Trait {}
impl<T: Inductive + Coinductive> Trait for T {}

trait Inductive {}
impl<T: Trait> Inductive for T {}
#[rustc_coinductive]
trait Coinductive {}
impl<T: Trait> Coinductive for T {}

fn impls_trait<T: Trait>() {}

#[rustc_coinductive]
trait TraitRev {}
impl<T: CoinductiveRev + InductiveRev> TraitRev for T {}

trait InductiveRev {}
impl<T: TraitRev> InductiveRev for T {}
#[rustc_coinductive]
trait CoinductiveRev {}
impl<T: TraitRev> CoinductiveRev for T {}

fn impls_trait_rev<T: TraitRev>() {}

fn main() {
    impls_trait::<()>();
// { dg-error ".E0275." "" { target *-*-* } .-1 }

    impls_trait_rev::<()>();
// { dg-error ".E0275." "" { target *-*-* } .-1 }
}


#![feature(coroutines, coroutine_trait, never_type)]

use std::ops::Coroutine;

fn mk_gen() -> impl Coroutine<Return=!, Yield=()> {
    #[coroutine] || { loop { yield; } }
}

fn main() {
    let gens: [impl Coroutine<Return=!, Yield=()>;2] = [ mk_gen(), mk_gen() ];
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}


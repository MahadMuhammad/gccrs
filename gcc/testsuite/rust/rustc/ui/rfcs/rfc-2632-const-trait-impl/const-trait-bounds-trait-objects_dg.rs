#![feature(const_trait_impl)]
// FIXME(effects) add effects
// { dg-additional-options "-frust-edition= 2021" }

#[const_trait]
trait Trait {}

fn main() {
    let _: &dyn const Trait; // { dg-error "" "" { target *-*-* } }
    let _: &dyn ~const Trait; // { dg-error "" "" { target *-*-* } }
}

// Regression test for issue #119525.
trait NonConst {}
const fn handle(_: &dyn const NonConst) {}
// { dg-error "" "" { target *-*-* } .-1 }
const fn take(_: &dyn ~const NonConst) {}
// { dg-error "" "" { target *-*-* } .-1 }


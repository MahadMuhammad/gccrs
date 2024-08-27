// This test is a collection of test that should pass.
//
//@ check-fail

#![feature(cfg_accessible)]
#![feature(trait_alias)]

trait TraitAlias = std::fmt::Debug + Send;

// FIXME: Currently shows "cannot determine" but should be `false`
#[cfg_accessible(unresolved)] // { dg-error "" "" { target *-*-* } }
const C: bool = true;

// FIXME: Currently shows "not sure" but should be `false`
#[cfg_accessible(TraitAlias::unresolved)] // { dg-error "" "" { target *-*-* } }
const D: bool = true;

fn main() {}


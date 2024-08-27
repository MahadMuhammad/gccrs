//@ aux-build:re-export.rs
// { dg-additional-options "-frust-edition=2018" }

extern crate re_export;

use re_export::cause_ice;

cause_ice!(); // { dg-error ".E0364." "" { target *-*-* } }

fn main() {}


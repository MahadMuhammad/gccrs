// test for ice #109178  cannot relate region: LUB(ReErased, ReError)

#![allow(incomplete_features)]
#![crate_type = "lib"]
#![feature(adt_const_params, unsized_const_params, generic_const_exprs)]

struct Changes<const CHANGES: &[&'static str]>
// { dg-error ".E0637." "" { target *-*-* } .-1 }
where
    [(); CHANGES.len()]:, {}

impl<const CHANGES: &[&str]> Changes<CHANGES> where [(); CHANGES.len()]: {}
// { dg-error ".E0637." "" { target *-*-* } .-1 }
// { dg-error ".E0637." "" { target *-*-* } .-2 }


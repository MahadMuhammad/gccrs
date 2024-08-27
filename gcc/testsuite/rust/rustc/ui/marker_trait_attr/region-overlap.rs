#![feature(marker_trait_attr)]

#[marker]
trait A {}
impl<'a> A for (&'static (), &'a ()) {} // { dg-error ".E0283." "" { target *-*-* } }
impl<'a> A for (&'a (), &'static ()) {} // { dg-error ".E0283." "" { target *-*-* } }

fn main() {}


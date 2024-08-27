#![feature(marker_trait_attr)]

#[marker]
trait Marker {}

impl Marker for &'_ () {} // { dg-error ".E0283." "" { target *-*-* } }
impl Marker for &'_ () {} // { dg-error ".E0283." "" { target *-*-* } }

fn main() {}


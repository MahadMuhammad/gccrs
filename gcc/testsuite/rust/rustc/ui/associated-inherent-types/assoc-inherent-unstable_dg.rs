//@ aux-crate:aux=assoc-inherent-unstable.rs
// { dg-additional-options "-frust-edition= 2021" }

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

type Data = aux::Owner::Data; // { dg-error ".E0658." "" { target *-*-* } }

fn main() {}


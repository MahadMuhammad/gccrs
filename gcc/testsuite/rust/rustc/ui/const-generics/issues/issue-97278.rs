#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::sync::Arc;

#[derive(PartialEq, Eq)]
enum Bar {
    Bar(Arc<i32>)
}

fn test<const BAR: Bar>() {}
// { dg-error ".E0741." "" { target *-*-* } .-1 }

fn main() {}


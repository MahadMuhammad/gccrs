#![feature(associated_type_defaults)]

use std::ops::Index;

trait Hierarchy {
    type Value;
    type ChildKey;
    type Children = dyn Index<Self::ChildKey, Output = dyn Hierarchy>;
// { dg-error ".E0191." "" { target *-*-* } .-1 }

    fn data(&self) -> Option<(Self::Value, Self::Children)>;
}

fn main() {}


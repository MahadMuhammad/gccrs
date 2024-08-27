#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    fn foo(&self) {}
}

struct F;
impl Trait for F {}
struct S(F);

pub mod to_reuse {
    use crate::F;

    pub fn foo(_: &F) {}
}

impl Trait for S {
    // Make sure that the method call is not generated if the path resolution
    // does not have a `self` parameter.
    reuse to_reuse::foo { self.0 }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}


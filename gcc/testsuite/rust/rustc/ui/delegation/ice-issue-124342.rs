#![feature(fn_delegation)]
#![allow(incomplete_features)]

mod to_reuse {}

trait Trait {
    reuse to_reuse::foo { foo }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
}

fn main() {}


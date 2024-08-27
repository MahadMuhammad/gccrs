#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    reuse Trait::foo { &self.0 }
// { dg-error "" "" { target *-*-* } .-1 }
}

// FIXME(fn_delegation): `recursive delegation` error should be emitted here
reuse foo;
// { dg-error ".E0391." "" { target *-*-* } .-1 }

fn main() {}


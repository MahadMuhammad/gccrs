#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    fn method() {}
}

reuse Trait::*; // { dg-error "" "" { target *-*-* } }

trait OtherTrait {
    reuse Trait::*; // { dg-error "" "" { target *-*-* } }
}

extern {
    reuse Trait::*; // { dg-error "" "" { target *-*-* } }
}

fn main() {
    reuse Trait::*; // { dg-error "" "" { target *-*-* } }
}


#![feature(staged_api)]
// { dg-error "" "" { target *-*-* } .-1 }

pub trait Trait {
// { dg-error "" "" { target *-*-* } .-1 }
    fn fun() {}
// { dg-error "" "" { target *-*-* } .-1 }
}

impl Trait for u8 {
// { dg-error "" "" { target *-*-* } .-1 }
    pub(self) fn fun() {}
// { dg-error ".E0449." "" { target *-*-* } .-1 }
}

fn main() {}


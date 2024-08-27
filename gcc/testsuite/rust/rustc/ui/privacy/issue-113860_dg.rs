#![feature(staged_api)]
// { dg-error "" "" { target *-*-* } .-1 }

pub trait Trait {
// { dg-error "" "" { target *-*-* } .-1 }
    const X: u32;
// { dg-error "" "" { target *-*-* } .-1 }
}

impl Trait for u8 {
// { dg-error "" "" { target *-*-* } .-1 }
    pub(self) const X: u32 = 3;
// { dg-error ".E0449." "" { target *-*-* } .-1 }
}

fn main() {}


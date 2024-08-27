#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    fn description(&self) -> &str {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

struct F;
struct S(F);

impl S {
    reuse <S as Trait>::description { &self.0 }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

fn main() {}


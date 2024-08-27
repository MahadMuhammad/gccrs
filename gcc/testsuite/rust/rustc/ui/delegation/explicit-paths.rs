#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    fn foo1(&self, x: i32) -> i32 { x }
    fn foo2(x: i32) -> i32 { x }
}

struct F;
impl Trait for F {}
struct S(F);

pub mod to_reuse {
    pub fn foo3() {}
}

impl F {
    fn foo4(&self) {}
}

mod fn_to_other {
    use super::*;

    reuse Trait::foo1;
    reuse <S as Trait>::foo2;
    reuse to_reuse::foo3;
    reuse S::foo4;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

mod inherent_impl_assoc_fn_to_other {
    use crate::*;

    impl S {
        reuse Trait::foo1 { self.0 }
        reuse <S as Trait>::foo2;
        reuse to_reuse::foo3;
        reuse F::foo4 { &self.0 }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
}

mod trait_impl_assoc_fn_to_other {
    use crate::*;

    impl Trait for S {
        reuse Trait::foo1 { self.0 }
        reuse <F as Trait>::foo2;
        reuse to_reuse::foo3;
// { dg-error ".E0407." "" { target *-*-* } .-1 }
        reuse F::foo4 { &self.0 }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
    }
}

mod trait_assoc_fn_to_other {
    use crate::*;

    trait Trait2 : Trait {
        reuse <F as Trait>::foo1 { self }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        reuse <F as Trait>::foo2;
        reuse to_reuse::foo3;
        reuse F::foo4 { &F }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
}

mod type_mismatch {
    use crate::*;

    struct S2;
    impl Trait for S {
// { dg-error ".E0119." "" { target *-*-* } .-1 }
        reuse <S2 as Trait>::foo1;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

fn main() {}


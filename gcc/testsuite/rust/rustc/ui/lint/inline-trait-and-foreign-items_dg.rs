#![feature(extern_types)]
#![feature(impl_trait_in_assoc_type)]

#![warn(unused_attributes)]

trait Trait {
    #[inline] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
    const X: u32;

    #[inline] // { dg-error ".E0518." "" { target *-*-* } }
    type T;

    type U;
}

impl Trait for () {
    #[inline] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
    const X: u32 = 0;

    #[inline] // { dg-error ".E0518." "" { target *-*-* } }
    type T = Self;

    #[inline] // { dg-error ".E0518." "" { target *-*-* } }
    type U = impl Trait; // { dg-error "" "" { target *-*-* } }
}

extern "C" {
    #[inline] // { dg-error ".E0518." "" { target *-*-* } }
    static X: u32;

    #[inline] // { dg-error ".E0518." "" { target *-*-* } }
    type T;
}

fn main() {}


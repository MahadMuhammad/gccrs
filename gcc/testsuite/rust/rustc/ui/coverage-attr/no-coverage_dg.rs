#![feature(extern_types)]
#![feature(coverage_attribute)]
#![feature(impl_trait_in_assoc_type)]
#![warn(unused_attributes)]
#![coverage(off)]

#[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
trait Trait {
    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    const X: u32;

    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    type T;

    type U;
}

#[coverage(off)]
impl Trait for () {
    const X: u32 = 0;

    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    type T = Self;

    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    type U = impl Trait; // { dg-error "" "" { target *-*-* } }
}

extern "C" {
    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    static X: u32;

    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    type T;
}

#[coverage(off)]
fn main() {
    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    let _ = ();

    match () {
        #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
        () => (),
    }

    #[coverage(off)] // { dg-error ".E0788." "" { target *-*-* } }
    return ();
}


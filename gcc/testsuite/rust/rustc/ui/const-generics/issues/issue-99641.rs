#![feature(adt_const_params)]
#![allow(incomplete_features)]

fn main() {
    pub struct Color<const WHITE: (fn(),)>;
// { dg-error ".E0741." "" { target *-*-* } .-1 }

    impl<const WHITE: (fn(),)> Color<WHITE> {
// { dg-error ".E0741." "" { target *-*-* } .-1 }
        pub fn new() -> Self {
            Color::<WHITE>
        }
    }

    pub const D65: (fn(),) = (|| {},);

    Color::<D65>::new();
}


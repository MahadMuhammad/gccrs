//@ aux-build:hidden.rs

extern crate hidden;

use hidden::HiddenStruct;

struct InCrate {
    a: usize,
    b: bool,
    #[doc(hidden)]
    im_hidden: u8
}

fn main() {
    let HiddenStruct { one, two } = HiddenStruct::default();
// { dg-error "" "" { target *-*-* } .-1 }

    let HiddenStruct { one } = HiddenStruct::default();
// { dg-error ".E0027." "" { target *-*-* } .-1 }

    let HiddenStruct { one, hide } = HiddenStruct::default();
// { dg-error ".E0027." "" { target *-*-* } .-1 }

    let InCrate { a, b } = InCrate { a: 0, b: false, im_hidden: 0 };
// { dg-error ".E0027." "" { target *-*-* } .-1 }
}


#![allow(unused_imports)]

mod inner {
    pub enum Example {
        ExOne,
    }
}

mod reexports {
    pub use crate::inner::Example as _;
}

use crate::reexports::*;
// { suggestion "" "" { target *-*-* } .-1 }

fn main() {
    ExOne;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}


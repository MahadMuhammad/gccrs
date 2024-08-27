#![deny(unused_imports)]

mod a {}

pub use a::*;
// { dg-error "" "" { target *-*-* } .-1 }

mod b {
    mod c {
        #[derive(Clone)]
        pub struct D;
    }
    pub use self::c::*; // don't show unused import lint
}

pub use b::*; // don't show unused import lint

mod d {
    const D: i32 = 1;
}

pub use d::*;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


//@ run-rustfix
//@ check-pass

#![warn(unused_imports)]

pub mod nested {
    pub struct A;
    pub struct B;
    pub struct C;
    pub struct D;
    pub mod even_more {
        pub struct E;
        pub struct F;
        pub struct G;
    }
    pub mod another {
        pub struct H;
        pub struct I;
    }
}

use nested::{A, B, C};
// { dg-warning "" "" { target *-*-* } .-1 }

use nested::{
    D,
    even_more::{
        E,
        F,
        G,
                         },
            };
// { dg-warning "" "" { target *-*-* } .-7 }

// Note that the following fix should result in `::{self}`, not `::self`. The latter is invalid
// Rust syntax, so the braces should not be removed.
use nested::another::{self, I};
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    let _ = (B, F, another::I);
}


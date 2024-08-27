mod m1 {
    pub use ::E::V; // { dg-error ".E0365." "" { target *-*-* } }
}

mod m2 {
    pub use ::E::{V}; // { dg-error ".E0365." "" { target *-*-* } }
}

mod m3 {
    pub use ::E::V::{self}; // { dg-error ".E0365." "" { target *-*-* } }
}

#[deny(unused_imports)]
mod m4 {
    pub use ::E::*;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

enum E { V }

fn main() {}


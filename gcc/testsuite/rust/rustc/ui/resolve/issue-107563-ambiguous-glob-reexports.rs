#![deny(ambiguous_glob_reexports)]

pub mod foo {
    pub type X = u8;
}

pub mod bar {
    pub type X = u8;
    pub type Y = u8;
}

pub use foo::*;
// { dg-error "" "" { target *-*-* } .-1 }
pub use bar::*;

mod ambiguous {
    mod m1 { pub type A = u8; }
    mod m2 { pub type A = u8; }
    pub use self::m1::*;
// { dg-error "" "" { target *-*-* } .-1 }
    pub use self::m2::*;
}

pub mod single {
    pub use ambiguous::A;
// { dg-error ".E0659." "" { target *-*-* } .-1 }
}

pub mod glob {
    pub use ambiguous::*;
}

pub fn main() {}


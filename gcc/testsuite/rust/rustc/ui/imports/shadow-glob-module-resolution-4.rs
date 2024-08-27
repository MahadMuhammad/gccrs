// https://github.com/rust-lang/rust/issues/126376

mod a {
    pub mod b {
        pub trait C {}
    }
}

use a::*;

use e as b;

use b::C as e;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-error ".E0432." "" { target *-*-* } .-2 }
// { dg-error ".E0432." "" { target *-*-* } .-3 }

fn e() {}

fn main() { }


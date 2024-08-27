// https://github.com/rust-lang/rust/issues/126389

mod a {
    pub mod b {
        pub mod c {}
    }
}

use a::*;

use b::c;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-error ".E0432." "" { target *-*-* } .-2 }
// { dg-error ".E0432." "" { target *-*-* } .-3 }
use c as b;

fn c() {}

fn main() { }


// https://github.com/rust-lang/rust/issues/125013

mod a {
  pub mod b {
    pub mod c {
      pub trait D {}
    }
  }
}

use a::*;

use e as b;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
use b::c::D as e;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() { }


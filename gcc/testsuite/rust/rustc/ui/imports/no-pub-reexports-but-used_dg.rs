//@ check-pass
// https://github.com/rust-lang/rust/issues/115966

mod m {
    pub(crate) type A = u8;
}

#[warn(unused_imports)] // { dg-note "" "" { target *-*-* } }
pub use m::*;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

fn main() {
    let _: A;
}


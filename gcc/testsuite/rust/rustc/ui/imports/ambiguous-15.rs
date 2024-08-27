//@ check-pass
// https://github.com/rust-lang/rust/pull/113099#issuecomment-1638206152

mod t2 {
    #[derive(Debug)]
    pub enum Error {}

    mod s {
        pub use std::fmt::*;
        pub trait Error: Sized {}
    }

    use self::s::*;
}

pub use t2::*;

mod t3 {
    pub trait Error {}
}

use self::t3::*;
fn a<E: Error>(_: E) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {}


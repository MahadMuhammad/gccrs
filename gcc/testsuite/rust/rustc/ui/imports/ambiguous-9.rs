//@ check-pass
// https://github.com/rust-lang/rust/pull/113099#issuecomment-1638206152

pub mod dsl {
    mod range {
        pub fn date_range() {}
    }
    pub use self::range::*; // { dg-warning "" "" { target *-*-* } }
    use super::prelude::*;
}

pub mod prelude {
    mod t {
      pub fn date_range() {}
    }
    pub use self::t::*; // { dg-warning "" "" { target *-*-* } }
    pub use super::dsl::*;
}

use dsl::*;
use prelude::*;

fn main() {
    date_range();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
}


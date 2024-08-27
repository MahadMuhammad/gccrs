//@ check-pass
// https://github.com/rust-lang/rust/pull/112743#issuecomment-1601986883

macro_rules! m {
    () => {
      pub fn id() {}
    };
}

mod openssl {
    pub use self::evp::*;
// { dg-warning "" "" { target *-*-* } .-1 }
    pub use self::handwritten::*;

    mod evp {
      m!();
    }

    mod handwritten {
      m!();
    }
}

pub use openssl::*;

fn main() {
    id();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}


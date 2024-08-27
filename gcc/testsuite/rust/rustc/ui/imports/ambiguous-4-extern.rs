//@ check-pass
// https://github.com/rust-lang/rust/pull/112743#issuecomment-1601986883

macro_rules! m {
    () => {
      pub fn id() {}
    };
}

pub use evp::*; // { dg-warning "" "" { target *-*-* } }
pub use handwritten::*;

mod evp {
    use *;
    m! {}
}
mod handwritten {
    use *;
    m! {}
}

fn main() {
    id();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}


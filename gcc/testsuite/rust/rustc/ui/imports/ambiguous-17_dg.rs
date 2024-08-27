//@ check-pass
// https://github.com/rust-lang/rust/pull/113099#issuecomment-1638206152

pub use evp::*; // { dg-warning "" "" { target *-*-* } }
pub use handwritten::*;

macro_rules! m {
    () => {
        pub fn id() {}
    };
}
mod evp {
    use *;
    m!();
}

mod handwritten {
    pub use handwritten::evp::*;
    mod evp {
        use *;
        m!();
    }
}

fn main() {
    id();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}


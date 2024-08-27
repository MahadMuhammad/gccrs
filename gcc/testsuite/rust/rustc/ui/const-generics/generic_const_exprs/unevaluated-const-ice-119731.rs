// rust-lang/rust#119731
// ICE ... unevaluated constant UnevaluatedConst

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod v20 {
    const v4: usize = 512;
    pub type v11 = [[usize; v4]; v4];
// { dg-warning "" "" { target *-*-* } .-1 }
    const v2: v11 = [[256; v4]; v4];

    const v0: [[usize; v4]; v4] = v6(v8);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
    pub struct v17<const v10: usize, const v7: v11> {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        _p: (),
    }

    impl v17<512, v0> {
        pub const fn v21() -> v18 {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    }

    impl<const v10: usize> v17<v10, v2> {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        pub const fn v21() -> v18 {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
            v18 { _p: () }
// { dg-error ".E0422." "" { target *-*-* } .-1 }
        }
    }
}
pub use v20::{v13, v17};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
fn main() {}


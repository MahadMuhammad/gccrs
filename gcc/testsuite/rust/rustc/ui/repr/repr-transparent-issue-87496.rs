// Regression test for the ICE described in #87496.

//@ check-pass

#[repr(transparent)]
struct TransparentCustomZst(());
extern "C" {
    fn good17(p: TransparentCustomZst);
// { dg-warning "" "" { target *-*-* } .-1 }
}

fn main() {}


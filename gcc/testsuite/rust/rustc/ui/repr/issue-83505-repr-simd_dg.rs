// Regression test for the ICE described in #83505.

#![crate_type="lib"]

#[repr(simd)]
// { dg-error ".E0084." "" { target *-*-* } .-1 }
// { dg-error ".E0084." "" { target *-*-* } .-2 }
enum Es {}
static CLs: Es;
// { dg-error "" "" { target *-*-* } .-1 }


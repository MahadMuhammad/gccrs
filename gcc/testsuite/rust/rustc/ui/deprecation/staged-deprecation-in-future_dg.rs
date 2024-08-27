#![deny(deprecated_in_future)]

#![feature(staged_api)]

#![stable(feature = "rustc_deprecation_in_future_test", since = "1.0.0")]

#[deprecated(since = "99.99.99", note = "effectively never")]
#[stable(feature = "rustc_deprecation_in_future_test", since = "1.0.0")]
pub struct S1;

#[deprecated(since = "TBD", note = "literally never")]
#[stable(feature = "rustc_deprecation_in_future_test", since = "1.0.0")]
pub struct S2;

fn main() {
    let _ = S1; // { dg-error "" "" { target *-*-* } }
    let _ = S2; // { dg-error "" "" { target *-*-* } }
}


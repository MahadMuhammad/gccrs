// Regression test for the ICE described in #83475.

#![crate_type="lib"]

#![feature(cmse_nonsecure_entry)]
#[cmse_nonsecure_entry]
// { dg-error "" "" { target *-*-* } .-1 }
struct XEmpty2;
// { dg-note "" "" { target *-*-* } .-1 }


#![crate_type = "lib"]

// This isn't intended to compile, so it's easiest to just ignore this error.
extern crate rustc_serialize; // { dg-error ".E0463." "" { target *-*-* } }

#[derive(
    RustcEncodable,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
    RustcDecodable,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
)]
struct S;


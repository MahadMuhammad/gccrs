// Top-level ill-formed
#[link] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
#[link = "foo"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
extern "C" {}

fn main() {}


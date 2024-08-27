#[link(name = "foo", cfg("rlib"))] // { dg-error "" "" { target *-*-* } }
extern "C" {}

fn main() {}


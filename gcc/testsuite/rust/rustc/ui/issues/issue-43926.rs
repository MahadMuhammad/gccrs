#[link(name = "foo", cfg())] // { dg-error "" "" { target *-*-* } }
extern "C" {}

fn main() {}


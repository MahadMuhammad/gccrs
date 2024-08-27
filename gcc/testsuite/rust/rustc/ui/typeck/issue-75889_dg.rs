// Regression test for #75889.

const FOO: dyn Fn() -> _ = ""; // { dg-error ".E0121." "" { target *-*-* } }
static BOO: dyn Fn() -> _ = ""; // { dg-error ".E0121." "" { target *-*-* } }

fn main() {}


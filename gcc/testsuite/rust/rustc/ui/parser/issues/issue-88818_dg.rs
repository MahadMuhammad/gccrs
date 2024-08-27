// Regression test for #88818 (improve error message for missing trait
// in `impl for X`).

struct S { }
impl for S { }
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

fn main() {}


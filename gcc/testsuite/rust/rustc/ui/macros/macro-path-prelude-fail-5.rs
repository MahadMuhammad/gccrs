#[derive(Clone, Debug)] // OK
struct S;

#[derive(Debug, inline)] // { dg-error "" "" { target *-*-* } }
struct T;

#[derive(inline, Debug)] // { dg-error "" "" { target *-*-* } }
struct U;

fn main() {}


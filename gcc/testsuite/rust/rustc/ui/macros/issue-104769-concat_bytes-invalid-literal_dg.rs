#![feature(concat_bytes)]

fn main() {
    concat_bytes!(7Y);
// { dg-error "" "" { target *-*-* } .-1 }
    concat_bytes!(888888888888888888888888888888888888888);
// { dg-error "" "" { target *-*-* } .-1 }
}


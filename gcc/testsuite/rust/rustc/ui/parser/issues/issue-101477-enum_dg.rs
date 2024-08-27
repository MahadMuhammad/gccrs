//@ run-rustfix

#[allow(dead_code)]
enum Demo {
    A = 1,
    B == 2 // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


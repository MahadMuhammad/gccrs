// { dg-additional-options "-frust-edition=2021" }

// Test that when a borrow checker diagnostics are emitted, it's as precise
// as the capture by the closure.

#![allow(unused)]

struct Point {
    x: i32,
    y: i32,
}
struct Wrapper {
    p: Point,
}

fn main() {
    let mut w = Wrapper { p: Point { x: 10, y: 10 } };

    let mut c = || {
        w.p.x += 20;
    };

    let py = &mut w.p.x;
// { dg-error ".E0499." "" { target *-*-* } .-1 }
    c();

    *py = 20
}


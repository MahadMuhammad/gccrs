//@ run-rustfix
#![deny(unused_variables)]

struct Point {
    x: u32,
    y: u32
}

fn process_point(Point { x, y: renamed }: Point) {
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x;
}

fn main() {
    process_point(Point { x: 0, y: 0 });
}


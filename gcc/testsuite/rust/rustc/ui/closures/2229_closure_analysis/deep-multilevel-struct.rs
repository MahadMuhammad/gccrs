// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]
#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Line {
    p: Point,
    q: Point
}
#[derive(Debug)]
struct Plane {
    a: Line,
    b: Line,
}

fn main() {
    let mut p = Plane {
        a: Line {
            p: Point { x: 1,y: 2 },
            q: Point { x: 3,y: 4 },
        },
        b: Line {
            p: Point { x: 1,y: 2 },
            q: Point { x: 3,y: 4 },
        }
    };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let x = &p.a.p.x;
// { dg-note "" "" { target *-*-* } .-1 }
        p.b.q.y = 9;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        println!("{:?}", p);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    };
}


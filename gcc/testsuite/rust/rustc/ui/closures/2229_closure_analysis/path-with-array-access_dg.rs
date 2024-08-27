// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

struct Point {
    x: f32,
    y: f32,
}

struct Pentagon {
    points: [Point; 5],
}

fn main() {
    let p1 = Point { x: 10.0, y: 10.0 };
    let p2 = Point { x: 7.5, y: 12.5 };
    let p3 = Point { x: 15.0, y: 15.0 };
    let p4 = Point { x: 12.5, y: 12.5 };
    let p5 = Point { x: 20.0, y: 10.0 };

    let pent = Pentagon { points: [p1, p2, p3, p4, p5] };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        println!("{}", pent.points[5].x);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };
}


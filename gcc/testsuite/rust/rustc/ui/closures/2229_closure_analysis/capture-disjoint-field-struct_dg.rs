// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 10, y: 10 };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        println!("{}", p.x);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    // `c` should only capture `p.x`, therefore mutating `p.y` is allowed.
    let py = &mut p.y;

    c();
    *py = 20;
}


// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

#[derive(Debug)]
struct Point {
    x: String,
    y: i32,
}

fn main() {
    let mut p = Point { x: String::new(), y: 10 };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let _x = p.x;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        println!("{:?}", p);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    };
}


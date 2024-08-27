// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

struct Point {
    x: i32,
    y: i32,
}

// This testcase ensures that nested closures are handles properly
// - The nested closure is analyzed first.
// - The capture kind of the nested closure is accounted for by the enclosing closure
// - Any captured path by the nested closure that starts off a local variable in the enclosing
// closure is not listed as a capture of the enclosing closure.

fn main() {
    let mut p = Point { x: 5, y: 20 };

    let mut c1 = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        println!("{}", p.x);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        let incr = 10;
        let mut c2 = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
        || p.y += incr;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-note "" "" { target *-*-* } .-8 }
        c2();
        println!("{}", p.y);
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c1();

    let px = &p.x;

    println!("{}", px);

    c1();
}


// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]
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

    // Only paths that appears within the closure that directly start off
    // a variable defined outside the closure are captured.
    //
    // Therefore `w.p` is captured
    // Note that `wp.x` doesn't start off a variable defined outside the closure.
    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let wp = &w.p;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        println!("{}", wp.x);
    };

    // Since `c` captures `w.p` by an ImmBorrow, `w.p.y` can't be mutated.
    let py = &mut w.p.y;
    c();

    *py = 20
}


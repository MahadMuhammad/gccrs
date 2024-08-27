// { dg-additional-options "-frust-edition=2021" }
#![feature(rustc_attrs)]

// Ensure that capture analysis results in arrays being completely captured.
fn main() {
    let mut m = [1, 2, 3, 4, 5];

    let mut c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        m[0] += 10;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        m[1] += 40;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}


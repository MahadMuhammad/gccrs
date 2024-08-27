// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

// Test to ensure that we can handle cases where
// let statements create no bindings are initialized
// using a Place expression
//
// Note: Currently when feature `capture_disjoint_fields` is enabled
// we can't handle such cases. So the test current use `_x` instead of
// `_` until the issue is resolved.
// Check rust-lang/project-rfc-2229#24 for status.

struct Point {
    x: i32,
    y: i32,
}

fn wild_struct() {
    let p = Point { x: 10, y: 20 };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        // FIXME(arora-aman): Change `_x` to `_`
        let Point { x: _x, y: _ } = p;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

fn wild_tuple() {
    let t = (String::new(), 10);

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        // FIXME(arora-aman): Change `_x` to `_`
        let (_x, _) = t;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

fn wild_arr() {
    let arr = [String::new(), String::new()];

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        // FIXME(arora-aman): Change `_x` to `_`
        let [_x, _] = arr;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

fn main() {}


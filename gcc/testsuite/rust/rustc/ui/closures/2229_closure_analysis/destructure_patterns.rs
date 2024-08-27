// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

// Test to ensure Index projections are handled properly during capture analysis
// The array should be moved in entirety, even though only some elements are used.
fn arrays() {
    let arr: [String; 5] = [format!("A"), format!("B"), format!("C"), format!("D"), format!("E")];

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let [a, b, .., e] = arr;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
        assert_eq!(a, "A");
        assert_eq!(b, "B");
        assert_eq!(e, "E");
    };

    c();
}

struct Point {
    x: i32,
    y: i32,
    id: String,
}

fn structs() {
    let mut p = Point { x: 10, y: 10, id: String::new() };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let Point { x: ref mut x, y: _, id: moved_id } = p;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

        println!("{}, {}", x, moved_id);
    };
    c();
}

fn tuples() {
    let mut t = (10, String::new(), (String::new(), 42));

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let (ref mut x, ref ref_str, (moved_s, _)) = t;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }

        println!("{}, {} {}", x, ref_str, moved_s);
    };
    c();
}

fn main() {}


// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

enum Info {
    Point(i32, i32, String),
    Meta(String, Vec<(i32, i32)>)
}

fn multi_variant_enum() {
    let point = Info::Point(10, -10, "1".into());

    let vec = Vec::new();
    let meta = Info::Meta("meta".into(), vec);

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        if let Info::Point(_, _, str) = point {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
            println!("{}", str);
        }

        if let Info::Meta(_, v) = meta {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
            println!("{:?}", v);
        }
    };

    c();
}

enum SingleVariant {
    Point(i32, i32, String),
}

fn single_variant_enum() {
    let point = SingleVariant::Point(10, -10, "1".into());

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let SingleVariant::Point(_, _, str) = point;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        println!("{}", str);
    };

    c();
}

fn main() {}


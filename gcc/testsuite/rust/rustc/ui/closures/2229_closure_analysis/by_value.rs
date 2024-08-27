// { dg-additional-options "-frust-edition=2021" }

// Test that we handle derferences properly when only some of the captures are being moved with
// `capture_disjoint_fields` enabled.
#![feature(rustc_attrs)]

#[derive(Debug, Default)]
struct SomeLargeType;
struct MuchLargerType([SomeLargeType; 32]);

// Ensure that we don't capture any derefs when moving captures into the closures,
// i.e. only data from the enclosing stack is moved.
fn big_box() {
    let s = MuchLargerType(Default::default());
    let b = Box::new(s);
    let t = (b, 10);

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let p = t.0.0;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        println!("{} {:?}", t.1, p);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

fn main() {
    big_box();
}


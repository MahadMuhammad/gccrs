// Matching against NaN should result in an error
#![allow(unused)]

const NAN: f64 = f64::NAN;

#[derive(PartialEq, Eq)]
struct MyType<T>(T);

const C: MyType<f32> = MyType(f32::NAN);

fn main() {
    let x = NAN;
    match x {
        NAN => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    };

    match [x, 1.0] {
        [NAN, _] => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    };

    match MyType(1.0f32) {
        C => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    }

    // Also cover range patterns
    match x {
        NAN..=1.0 => {}, // { dg-error "" "" { target *-*-* } }
        -1.0..=NAN => {}, // { dg-error "" "" { target *-*-* } }
        NAN.. => {}, // { dg-error "" "" { target *-*-* } }
        ..NAN => {}, // { dg-error "" "" { target *-*-* } }
        _ => {},
    };
}


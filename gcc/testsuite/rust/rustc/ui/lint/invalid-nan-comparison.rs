//@ check-pass

fn main() {
    f32();
    f64();
}

const TEST: bool = 5f32 == f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }

fn f32() {
    macro_rules! number { () => { 5f32 }; }
    let x = number!();
    x == f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x != f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x < f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x > f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x <= f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x >= f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    number!() == f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    f32::NAN != number!();
// { dg-warning "" "" { target *-*-* } .-1 }
}

fn f64() {
    macro_rules! number { () => { 5f64 }; }
    let x = number!();
    x == f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x != f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x < f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x > f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x <= f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    x >= f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    number!() == f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    f64::NAN != number!();
// { dg-warning "" "" { target *-*-* } .-1 }
}


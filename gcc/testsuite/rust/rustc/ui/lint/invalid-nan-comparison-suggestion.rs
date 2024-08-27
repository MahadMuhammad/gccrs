//@ check-pass
//@ run-rustfix

fn main() {
    let x = 5f32;
    let _ = x == f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    let _ = x != f32::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }

    let x = 5f64;
    let _ = x == f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }
    let _ = x != f64::NAN;
// { dg-warning "" "" { target *-*-* } .-1 }

    let b = &2.3f32;
    if b != &f32::NAN {}
// { dg-warning "" "" { target *-*-* } .-1 }

    let b = &2.3f32;
    if b != { &f32::NAN } {}
// { dg-warning "" "" { target *-*-* } .-1 }

    let _ =
        b != {
// { dg-warning "" "" { target *-*-* } .-1 }
            &f32::NAN
        };

    #[allow(unused_macros)]
    macro_rules! nan { () => { f32::NAN }; }
    macro_rules! number { () => { 5f32 }; }

    let _ = nan!() == number!();
// { dg-warning "" "" { target *-*-* } .-1 }
    let _ = number!() != nan!();
// { dg-warning "" "" { target *-*-* } .-1 }
}


//@ check-fail

#![deny(ambiguous_negative_literals)]

fn main() {
    let _ = -1i32.abs();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1f32.abs();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1f64.asin();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1f64.asinh();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1f64.tan();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1f64.tanh();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1.0_f64.cos().cos();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1.0_f64.cos().sin();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1.0_f64.sin().cos();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = -1f64.sin().sin();
// { dg-error "" "" { target *-*-* } .-1 }

    dbg!( -1.0_f32.cos() );
// { dg-error "" "" { target *-*-* } .-1 }

    // should not warn
    let _ = (-1i32).abs();
    let _ = (-1f32).abs();
    let _ = -(1i32).abs();
    let _ = -(1f32).abs();
    let _ = -(1i32.abs());
    let _ = -(1f32.abs());
}


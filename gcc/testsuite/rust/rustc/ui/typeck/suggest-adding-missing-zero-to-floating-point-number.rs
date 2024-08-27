//@ run-rustfix

fn main() {
    2.e1; // { dg-error ".E0610." "" { target *-*-* } }
    2.E1; // { dg-error ".E0610." "" { target *-*-* } }
    2.f32; // { dg-error ".E0610." "" { target *-*-* } }
    2.f64; // { dg-error ".E0610." "" { target *-*-* } }
    2.e+12; // { dg-error ".E0610." "" { target *-*-* } }
    2.e-12; // { dg-error ".E0610." "" { target *-*-* } }
    2.e1f32; // { dg-error ".E0610." "" { target *-*-* } }
}


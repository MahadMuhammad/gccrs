struct X {}

fn two_args(_a: i32, _b: f32) {}
fn three_args(_a: i32, _b: f32, _c: &str) {}
fn four_args(_a: i32, _b: f32, _c: &str, _d: X) {}

fn main() {
  two_args(1.0, 1); // { dg-error ".E0308." "" { target *-*-* } }
  three_args(1.0,   1,  ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_args(  1,  "", 1.0); // { dg-error ".E0308." "" { target *-*-* } }
  three_args( "", 1.0,   1); // { dg-error ".E0308." "" { target *-*-* } }

  four_args(1.0, 1, X {}, ""); // { dg-error ".E0308." "" { target *-*-* } }
}


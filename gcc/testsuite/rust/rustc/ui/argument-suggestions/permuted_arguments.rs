// More complicated permutations
struct X {}
struct Y {}

fn three_args(_a: i32, _b: f32, _c: &str) {}
fn many_args(_a: i32, _b: f32, _c: &str, _d: X, _e: Y) {}

fn main() {
  // b, c, a
  three_args(1.0, "", 1); // { dg-error ".E0308." "" { target *-*-* } }
  // d, e, b, a, c
  many_args(X {}, Y {}, 1, 1.0, ""); // { dg-error ".E0308." "" { target *-*-* } }
}


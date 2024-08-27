// Cases where multiple argument suggestions are mixed

struct X {}

fn two_args(_a: i32, _b: f32) {}
fn three_args(_a: i32, _b: f32, _c: &str) {}

fn main() {
  // Extra + Invalid
  two_args(1, "", X {}); // { dg-error ".E0061." "" { target *-*-* } }
  three_args(1, "", X {}, ""); // { dg-error ".E0061." "" { target *-*-* } }

  // Missing and Invalid
  three_args(1, X {}); // { dg-error ".E0061." "" { target *-*-* } }

  // Missing and Extra
  three_args(1, "", X {}); // { dg-error ".E0308." "" { target *-*-* } }

  // Swapped and Invalid
  three_args("", X {}, 1); // { dg-error ".E0308." "" { target *-*-* } }

  // Swapped and missing
  three_args("", 1); // { dg-error ".E0061." "" { target *-*-* } }
}


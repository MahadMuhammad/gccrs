// More nuanced test cases for invalid arguments #65853

struct X {}

fn one_arg(_a: i32) {}
fn two_arg_same(_a: i32, _b: i32) {}
fn two_arg_diff(_a: i32, _b: f32) {}
fn three_arg_diff(_a: i32, _b: f32, _c: &str) {}
fn three_arg_repeat(_a: i32, _b: i32, _c: &str) {}

fn main() {
  // Providing an incorrect argument for a single parameter function
  one_arg(1.0); // { dg-error ".E0308." "" { target *-*-* } }

  // Providing one or two invalid arguments to a two parameter function
  two_arg_same(1, ""); // { dg-error ".E0308." "" { target *-*-* } }
  two_arg_same("", 1); // { dg-error ".E0308." "" { target *-*-* } }
  two_arg_same("", ""); // { dg-error ".E0308." "" { target *-*-* } }
  two_arg_diff(1, ""); // { dg-error ".E0308." "" { target *-*-* } }
  two_arg_diff("", 1.0); // { dg-error ".E0308." "" { target *-*-* } }
  two_arg_diff("", ""); // { dg-error ".E0308." "" { target *-*-* } }

  // Providing invalid arguments to a three parameter function
  three_arg_diff(X{}, 1.0, ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_diff(1, X {}, ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_diff(1, 1.0, X {}); // { dg-error ".E0308." "" { target *-*-* } }

  three_arg_diff(X {}, X {}, ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_diff(X {}, 1.0, X {}); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_diff(1, X {}, X {}); // { dg-error ".E0308." "" { target *-*-* } }

  three_arg_diff(X {}, X {}, X {}); // { dg-error ".E0308." "" { target *-*-* } }

  three_arg_repeat(X {}, 1, ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_repeat(1, X {}, ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_repeat(1, 1, X {}); // { dg-error ".E0308." "" { target *-*-* } }

  three_arg_repeat(X {}, X {}, ""); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_repeat(X {}, 1, X {}); // { dg-error ".E0308." "" { target *-*-* } }
  three_arg_repeat(1, X {}, X{}); // { dg-error ".E0308." "" { target *-*-* } }

  three_arg_repeat(X {}, X {}, X {}); // { dg-error ".E0308." "" { target *-*-* } }
}


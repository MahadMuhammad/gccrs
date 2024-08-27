fn empty() {}
fn one_arg<T>(_a: T) {}
fn two_arg_same(_a: i32, _b: i32) {}
fn two_arg_diff(_a: i32, _b: &str) {}

macro_rules! foo {
    ($x:expr, ~) => {
        empty($x, 1); // { dg-error ".E0061." "" { target *-*-* } }
    };
    ($x:expr, $y:expr) => {
        empty($x, $y); // { dg-error ".E0061." "" { target *-*-* } }
    };
    (~, $y:expr) => {
        empty(1, $y); // { dg-error ".E0061." "" { target *-*-* } }
    };
}

fn main() {
  empty(""); // { dg-error ".E0061." "" { target *-*-* } }
  empty(1, 1); // { dg-error ".E0061." "" { target *-*-* } }

  one_arg(1, 1); // { dg-error ".E0061." "" { target *-*-* } }
  one_arg(1, ""); // { dg-error ".E0061." "" { target *-*-* } }
  one_arg(1, "", 1.0); // { dg-error ".E0061." "" { target *-*-* } }

  two_arg_same(1, 1, 1); // { dg-error ".E0061." "" { target *-*-* } }
  two_arg_same(1, 1, 1.0); // { dg-error ".E0061." "" { target *-*-* } }

  two_arg_diff(1, 1, ""); // { dg-error ".E0061." "" { target *-*-* } }
  two_arg_diff(1, "", ""); // { dg-error ".E0061." "" { target *-*-* } }
  two_arg_diff(1, 1, "", ""); // { dg-error ".E0061." "" { target *-*-* } }
  two_arg_diff(1, "", 1, ""); // { dg-error ".E0061." "" { target *-*-* } }

  // Check with weird spacing and newlines
  two_arg_same(1, 1,     ""); // { dg-error ".E0061." "" { target *-*-* } }
  two_arg_diff(1, 1,     ""); // { dg-error ".E0061." "" { target *-*-* } }
  two_arg_same( // { dg-error ".E0061." "" { target *-*-* } }
    1,
    1,
    ""
  );

  two_arg_diff( // { dg-error ".E0061." "" { target *-*-* } }
    1,
    1,
    ""
  );

  // Check with macro expansions
  foo!(1, ~);
  foo!(~, 1);
  foo!(1, 1);
  one_arg(1, panic!()); // { dg-error ".E0061." "" { target *-*-* } }
  one_arg(panic!(), 1); // { dg-error ".E0061." "" { target *-*-* } }
  one_arg(stringify!($e), 1); // { dg-error ".E0061." "" { target *-*-* } }

  // Not a macro, but this also has multiple spans with equal source code,
  // but different expansion contexts.
  // https://github.com/rust-lang/rust/issues/114255
  one_arg(for _ in 1.. {}, 1); // { dg-error ".E0061." "" { target *-*-* } }
}


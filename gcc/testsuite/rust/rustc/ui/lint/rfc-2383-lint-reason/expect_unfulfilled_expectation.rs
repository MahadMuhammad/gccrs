//@ check-pass
// ignore-tidy-linelength

#![warn(unused_mut)]

#![expect(unfulfilled_lint_expectations, reason = "idk why you would expect this")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

#[expect(unfulfilled_lint_expectations, reason = "a local: idk why you would expect this")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
pub fn normal_test_fn() {
    #[expect(unused_mut, reason = "this expectation will create a diagnostic with the default lint level")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    let mut v = vec![1, 1, 2, 3, 5];
    v.sort();

    // Check that lint lists including `unfulfilled_lint_expectations` are also handled correctly
    #[expect(unused, unfulfilled_lint_expectations, reason = "the expectation for `unused` should be fulfilled")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    let value = "I'm unused";
}

#[expect(warnings, reason = "this suppresses all warnings and also suppresses itself. No warning will be issued")]
pub fn expect_warnings() {
    // This lint trigger will be suppressed
    #[warn(unused_mut)]
    let mut v = vec![1, 1, 2, 3, 5];
}

fn main() {}


// Regression test for issue #87199, where attempting to relax a bound
// other than the only supported `?Sized` would still cause the compiler
// to assume that the `Sized` bound was relaxed.

//@ check-fail

// Check that these function definitions only emit warnings, not errors
fn arg<T: ?Send>(_: T) {}
// { dg-warning "" "" { target *-*-* } .-1 }
fn ref_arg<T: ?Send>(_: &T) {}
// { dg-warning "" "" { target *-*-* } .-1 }
fn ret() -> impl Iterator<Item = ()> + ?Send { std::iter::empty() }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

// Check that there's no `?Sized` relaxation!
fn main() {
    ref_arg::<i32>(&5);
    ref_arg::<[i32]>(&[5]);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


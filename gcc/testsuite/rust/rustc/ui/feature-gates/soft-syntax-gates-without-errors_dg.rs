//@ check-pass
// This file is used to test the behavior of the early-pass syntax warnings.
// If macro syntax is stabilized, replace with a different unstable syntax.

#[cfg(FALSE)]
macro b() {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

macro_rules! identity {
    ($($x:tt)*) => ($($x)*);
}

#[cfg(FALSE)]
identity! {
    macro d() {} // No error
}

identity! {
    #[cfg(FALSE)]
    macro e() {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

fn main() {}


//@ check-fail
// This file is used to test the behavior of the early-pass syntax warnings.
// If macro syntax is stabilized, replace with a different unstable syntax.

macro a() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

#[cfg(FALSE)]
macro b() {}

macro_rules! identity {
    ($($x:tt)*) => ($($x)*);
}

identity! {
    macro c() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

#[cfg(FALSE)]
identity! {
    macro d() {} // No error
}

identity! {
    #[cfg(FALSE)]
    macro e() {}
}

fn main() {}


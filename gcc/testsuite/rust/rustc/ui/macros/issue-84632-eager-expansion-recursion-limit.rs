// Regression test for #84632: Recursion limit is ignored
// for builtin macros that eagerly expands.

#![recursion_limit = "15"]
macro_rules! a {
    () => ("");
    (A) => (concat!("", a!()));
    (A, $($A:ident),*) => (concat!("", a!($($A),*)))
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {
    a!(A, A, A, A, A);
    a!(A, A, A, A, A, A, A, A, A, A, A);
}


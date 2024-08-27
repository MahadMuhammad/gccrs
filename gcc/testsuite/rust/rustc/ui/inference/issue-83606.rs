// Regression test for #83606.

fn foo<const N: usize>(_: impl std::fmt::Display) -> [usize; N] {
    [0; N]
}

fn main() {
    let _ = foo("foo");
// { dg-error ".E0284." "" { target *-*-* } .-1 }
}


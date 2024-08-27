fn a() where for<T> T: Copy {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn b() where for<const C: usize> [(); C]: Copy {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}


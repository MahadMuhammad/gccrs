enum Quux<T> { Bar }
// { dg-error ".E0392." "" { target *-*-* } .-1 }

fn foo(c: Quux) { assert!((false)); } // { dg-error ".E0107." "" { target *-*-* } }

fn main() { panic!(); }


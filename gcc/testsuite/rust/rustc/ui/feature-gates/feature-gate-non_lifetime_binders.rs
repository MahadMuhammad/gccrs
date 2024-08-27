fn foo() where for<T> T:, {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}


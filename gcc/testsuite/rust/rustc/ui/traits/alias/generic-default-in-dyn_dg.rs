trait SendEqAlias<T> = PartialEq;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

struct Foo<T>(dyn SendEqAlias<T>);
// { dg-error ".E0393." "" { target *-*-* } .-1 }

struct Bar<T>(dyn SendEqAlias<T>, T);
// { dg-error ".E0393." "" { target *-*-* } .-1 }

fn main() {}


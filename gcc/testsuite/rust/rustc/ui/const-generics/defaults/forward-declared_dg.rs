struct Foo<const N: usize = M, const M: usize = 10>;
// { dg-error ".E0128." "" { target *-*-* } .-1 }

enum Bar<const N: usize = M, const M: usize = 10> {}
// { dg-error ".E0128." "" { target *-*-* } .-1 }

struct Foo2<const N: usize = N>;
// { dg-error ".E0128." "" { target *-*-* } .-1 }

enum Bar2<const N: usize = N> {}
// { dg-error ".E0128." "" { target *-*-* } .-1 }

fn main() {}


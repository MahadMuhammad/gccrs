fn avg<T=T::Item>(_: T) {}
// { dg-error ".E0128." "" { target *-*-* } .-1 }
// { dg-error ".E0128." "" { target *-*-* } .-2 }
// { dg-warning ".E0128." "" { target *-*-* } .-3 }

fn main() {}


use std::marker::PhantomData;

struct B<T, const N: T>(PhantomData<[T; N]>);
// { dg-error ".E0770." "" { target *-*-* } .-1 }

fn main() {}


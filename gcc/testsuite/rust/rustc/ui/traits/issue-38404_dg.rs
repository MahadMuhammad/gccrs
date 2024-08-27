trait A<T>: std::ops::Add<Self> + Sized {}
trait B<T>: A<T> {}
trait C<T>: A<dyn B<T, Output = usize>> {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }

fn main() {}


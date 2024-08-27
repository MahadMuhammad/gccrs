struct A<T> {
// { dg-error ".E0072." "" { target *-*-* } .-1 }
    x: T,
    y: B<T>,
}

struct B<T> {
    z: A<T>
}

struct C<T> {
// { dg-error ".E0072." "" { target *-*-* } .-1 }
    x: T,
    y: Option<Option<D<T>>>,
}

struct D<T> {
    z: Option<Option<C<T>>>,
}

fn main() {}


struct A<T> {
// { dg-error ".E0072." "" { target *-*-* } .-1 }
    x: T,
    y: A<A<T>>,
}

struct B {
    z: A<usize>
}

fn main() {}


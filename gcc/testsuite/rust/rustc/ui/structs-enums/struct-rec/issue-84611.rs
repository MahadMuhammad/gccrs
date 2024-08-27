struct Foo<T> {
// { dg-error ".E0072." "" { target *-*-* } .-1 }
    x: Foo<[T; 1]>,
    y: T,
}

struct Bar {
    x: Foo<Bar>,
}

fn main() {}


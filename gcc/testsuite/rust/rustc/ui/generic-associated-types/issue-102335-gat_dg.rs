trait T {
    type A: S<C<(), i32 = ()> = ()>;
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
}

trait Q {}

trait S {
    type C<T>: Q;
}

fn main() {}


trait A {
    type T;
}

trait B: A<T: B> {}
// { dg-error ".E0391." "" { target *-*-* } .-1 }

fn main() {}


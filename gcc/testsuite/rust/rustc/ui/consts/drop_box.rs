const fn f<T>(_: Box<T>) {}
// { dg-error ".E0493." "" { target *-*-* } .-1 }

fn main() {}


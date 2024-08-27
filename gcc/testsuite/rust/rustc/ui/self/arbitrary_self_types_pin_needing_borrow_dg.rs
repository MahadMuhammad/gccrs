use std::pin::Pin;
struct S;

impl S {
    fn x(self: Pin<&mut Self>) {
    }
}

fn main() {
    Pin::new(S).x();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
}


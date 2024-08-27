//@ run-rustfix
use std::pin::Pin;
struct S;

impl S {
    fn x(self: Pin<&mut Self>) {
    }
}

fn main() {
    S.x(); // { dg-error ".E0599." "" { target *-*-* } }
}


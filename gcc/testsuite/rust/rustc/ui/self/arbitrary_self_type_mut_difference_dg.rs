// Related to #57994.
use std::pin::Pin;
struct S;

impl S {
    fn x(self: Pin<&mut Self>) {} // { dg-note "" "" { target *-*-* } }
    fn y(self: Pin<&Self>) {} // { dg-note "" "" { target *-*-* } }
}

fn main() {
    Pin::new(&S).x(); // { dg-error ".E0599." "" { target *-*-* } }
    Pin::new(&mut S).y(); // { dg-error ".E0599." "" { target *-*-* } }
}


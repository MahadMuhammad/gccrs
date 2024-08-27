pub struct A;

fn main() {
    match () {
        _ => match A::partial_cmp() {},
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    }
}


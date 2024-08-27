fn f() -> impl Sized {
    2.0E
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


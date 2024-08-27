//@ check-pass

trait T {}

fn wrap(x: impl T) -> impl T {
// { dg-warning "" "" { target *-*-* } .-1 }
    wrap(wrap(x))
}

fn main() {}


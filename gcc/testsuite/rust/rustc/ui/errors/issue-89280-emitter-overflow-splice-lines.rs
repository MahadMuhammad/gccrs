//@ check-pass

trait X {
    fn test(x: u32, (
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    )) {}
}

fn main() {}


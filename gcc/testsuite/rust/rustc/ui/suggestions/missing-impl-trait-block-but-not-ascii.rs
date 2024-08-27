// issue#126764

struct S;

trait T {
    fn f();
}
impl T for S；
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
// { dg-error ".E0046." "" { target *-*-* } .-3 }

fn main() {}


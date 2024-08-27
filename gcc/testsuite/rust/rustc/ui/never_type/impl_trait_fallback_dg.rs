//@ check-pass

fn main() {}

trait T {}
impl T for () {}

fn should_ret_unit() -> impl T {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    panic!()
}


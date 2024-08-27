//@ run-rustfix
fn main() {
    f()  : // { dg-error "" "" { target *-*-* } }
    f();
}

fn f() {}


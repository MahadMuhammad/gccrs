//@ run-rustfix

fn f() {}
fn i(_: u32) {}
fn is(_: u32, _: &str) {}
fn s(_: &str) {}

fn main() {
    // code             expected suggestion
    f(0, 1,);        // f()
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    i(0, 1, 2,);     // i(0,)
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    i(0, 1, 2);      // i(0)
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    is(0, 1, 2, ""); // is(0, "")
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    s(0, 1, "");     // s("")
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}


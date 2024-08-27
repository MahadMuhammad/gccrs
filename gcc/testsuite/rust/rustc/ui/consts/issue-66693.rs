// Tests that the compiler does not ICE when const-evaluating a `panic!()` invocation with a
// non-`&str` argument.

const _: () = panic!(1);
// { dg-error "" "" { target *-*-* } .-1 }

static _FOO: () = panic!(true);
// { dg-error "" "" { target *-*-* } .-1 }

const fn _foo() {
    panic!(&1);
// { dg-error "" "" { target *-*-* } .-1 }
}

// ensure that conforming panics don't cause an error beyond the failure to const eval
const _: () = panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }
static _BAR: () = panic!("panic in static");
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const fn _bar() {
    panic!("panic in const fn");
}

fn main() {}


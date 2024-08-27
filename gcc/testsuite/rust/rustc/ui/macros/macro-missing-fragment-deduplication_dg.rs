//@ compile-flags: -Zdeduplicate-diagnostics=yes

macro_rules! m {
    ($name) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
}

fn main() {
    m!();
    m!();
    m!();
    m!();
}


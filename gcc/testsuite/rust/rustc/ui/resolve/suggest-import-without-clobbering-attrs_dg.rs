//@ run-rustfix
//@ compile-flags: -Aunused

#[cfg(all())]
use y::Whatever;

mod y {
    pub(crate) fn z() {}
    pub(crate) struct Whatever;
}

fn main() {
    z();
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}


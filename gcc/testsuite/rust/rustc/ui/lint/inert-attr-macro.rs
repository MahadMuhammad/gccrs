//@ check-pass

#![warn(unused)]

macro_rules! foo {
    () => {}
}

fn main() {
    #[inline] foo!(); // { dg-warning "" "" { target *-*-* } }

    // This does nothing, since `#[allow(warnings)]` is itself
    // an inert attribute on a macro call
    #[allow(warnings)] #[inline] foo!(); // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }

    // This does work, since the attribute is on a parent
    // of the macro invocation.
    #[allow(warnings)] { #[inline] foo!(); }
}


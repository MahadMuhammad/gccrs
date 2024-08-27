//@ check-fail

#![deny(non_exhaustive_omitted_patterns)]
// { dg-warning "" "" { target *-*-* } .-1 }
#![allow(non_exhaustive_omitted_patterns)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    enum Foo {
        A,
        B,
        C,
    }

    #[allow(non_exhaustive_omitted_patterns)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    match Foo::A {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        Foo::A => {}
        Foo::B => {}
    }

    #[warn(non_exhaustive_omitted_patterns)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    match Foo::A {
        Foo::A => {}
        Foo::B => {}
        _ => {}
    }
}


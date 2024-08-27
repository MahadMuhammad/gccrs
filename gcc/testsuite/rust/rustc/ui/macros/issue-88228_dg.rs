//@ compile-flags: -Z deduplicate-diagnostics=yes
// { dg-additional-options "-frust-edition=2018" }

mod hey { // { help "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    pub use Copy as Bla;
    pub use std::println as bla;
}

#[derive(Bla)]
// { dg-error "" "" { target *-*-* } .-1 }
struct A;

#[derive(println)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
struct B;

fn main() {
    bla!();
// { dg-error "" "" { target *-*-* } .-1 }
}


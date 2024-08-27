//@ compile-flags: -Zverbose-internals
// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

fn main() {
    let x = async || {};
// { dg-note "" "" { target *-*-* } .-1 }
    let () = x();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
}


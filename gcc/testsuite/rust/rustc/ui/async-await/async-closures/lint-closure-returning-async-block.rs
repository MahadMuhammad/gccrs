// { dg-additional-options "-frust-edition= 2021" }

#![feature(async_closure)]
#![deny(closure_returning_async_block)]

fn main() {
    let x = || async {};
// { dg-error "" "" { target *-*-* } .-1 }

    let x = || async move {};
// { dg-error "" "" { target *-*-* } .-1 }

    let x = move || async move {};
// { dg-error "" "" { target *-*-* } .-1 }

    let x = move || async {};
// { dg-error "" "" { target *-*-* } .-1 }

    let x = || {{ async {} }};
// { dg-error "" "" { target *-*-* } .-1 }
}


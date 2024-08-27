// Check that using the parameter name in its type does not ICE.
// { dg-additional-options "-frust-edition=2018" }

#![feature(async_closure)]

fn main() {
    let _ = |x: x| x; // { dg-error ".E0573." "" { target *-*-* } }
    let _ = |x: bool| -> x { x }; // { dg-error ".E0573." "" { target *-*-* } }
    let _ = async move |x: x| x; // { dg-error ".E0573." "" { target *-*-* } }
    let _ = async move |x: bool| -> x { x }; // { dg-error ".E0573." "" { target *-*-* } }
}

fn foo(x: x) {} // { dg-error ".E0573." "" { target *-*-* } }
fn foo_ret(x: bool) -> x {} // { dg-error ".E0573." "" { target *-*-* } }

async fn async_foo(x: x) {} // { dg-error ".E0573." "" { target *-*-* } }
async fn async_foo_ret(x: bool) -> x {} // { dg-error ".E0573." "" { target *-*-* } }


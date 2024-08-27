//@ run-rustfix
#![allow(dead_code, unused_variables, path_statements)]
fn a() {
    let x = 5;
    let y = x // { dg-error ".E0618." "" { target *-*-* } }
    () // { dg-error "" "" { target *-*-* } }
}

fn b() {
    let x = 5;
    let y = x // { dg-error ".E0618." "" { target *-*-* } }
    ();
}
fn c() {
    let x = 5;
    x // { dg-error ".E0618." "" { target *-*-* } }
    ()
}
fn d() { // ok
    let x = || ();
    x
    ()
}
fn e() { // ok
    let x = || ();
    x
    ();
}
fn f()
 {
    let y = 5 // { dg-error ".E0618." "" { target *-*-* } }
    () // { dg-error "" "" { target *-*-* } }
}
fn g() {
    5 // { dg-error ".E0618." "" { target *-*-* } }
    ();
}
fn main() {}


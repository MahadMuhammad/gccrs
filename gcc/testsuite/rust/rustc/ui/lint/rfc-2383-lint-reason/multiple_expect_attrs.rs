//@ check-pass

#![warn(unused)]

#[warn(unused_variables)]
#[expect(unused_variables)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
#[allow(unused_variables)]
#[expect(unused_variables)] // Only this expectation will be fulfilled
fn main() {
    let x = 2;
}


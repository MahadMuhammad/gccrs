// Check what happens when we forbid a group but
// then allow a member of that group.
//
//@ check-pass

#![forbid(unused)]

#[allow(unused_variables)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
fn main() {
    let a: ();
}


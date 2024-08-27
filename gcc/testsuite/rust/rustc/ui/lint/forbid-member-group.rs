// Check what happens when we forbid a member of
// a group but then allow the group.

#![forbid(unused_variables)]

#[allow(unused)]
// { dg-error ".E0453." "" { target *-*-* } .-1 }
fn main() {
    let a: ();
}


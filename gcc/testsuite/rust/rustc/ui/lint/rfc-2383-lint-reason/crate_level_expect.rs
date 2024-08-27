//@ check-pass

#![warn(unused)]

#![expect(unused_mut)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

#![expect(unused_variables)]

fn main() {
    let x = 0;
}


#![warn(unused)]
#![deny(warnings)]

fn main() {
    let _x: ([u32; 3]); // { dg-error "" "" { target *-*-* } }
    let _y: [u8; (3)]; // { dg-error "" "" { target *-*-* } }
    let _z: ([u8; (3)]);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

}


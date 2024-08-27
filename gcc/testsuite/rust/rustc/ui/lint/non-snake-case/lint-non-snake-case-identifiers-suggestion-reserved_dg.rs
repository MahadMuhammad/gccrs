#![warn(unused)]
#![allow(dead_code)]
#![deny(non_snake_case)]

mod Impl {}
// { dg-error "" "" { target *-*-* } .-1 }

fn While() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    let Mod: usize = 0;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    let Super: usize = 0;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}


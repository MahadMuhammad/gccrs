//@ check-pass

#![warn(unused_must_use)]
#![feature(never_type)]

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

fn main() {
    let x = 2_u32;
    (x.add(4), x.sub(4), x.mul(4), x.div(4), x.rem(4));

    x.add(4); // { dg-warning "" "" { target *-*-* } }

    x.sub(4); // { dg-warning "" "" { target *-*-* } }

    x.mul(4); // { dg-warning "" "" { target *-*-* } }

    x.div(4); // { dg-warning "" "" { target *-*-* } }

    x.rem(4); // { dg-warning "" "" { target *-*-* } }

    println!("{}", x);
}


// Regression test for #82865.

#![feature(decl_macro)]

use x::y::z; // { dg-error ".E0433." "" { target *-*-* } }

macro mac () {
    Box::z // { dg-error ".E0599." "" { target *-*-* } }
}

fn main() {
    mac!();
}


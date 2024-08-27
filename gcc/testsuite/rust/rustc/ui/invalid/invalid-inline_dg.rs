#![allow(dead_code)]

#[inline(please,no)] // { dg-error ".E0534." "" { target *-*-* } }
fn a() {
}

#[inline()] // { dg-error ".E0534." "" { target *-*-* } }
fn b() {
}

fn main() {
    a();
    b();
}


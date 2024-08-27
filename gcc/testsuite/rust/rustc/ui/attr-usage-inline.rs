#![allow(dead_code)]

#[inline]
fn f() {}

#[inline] // { dg-error ".E0518." "" { target *-*-* } }
struct S;

struct I {
    #[inline]
    i: u8,
}

#[macro_export]
#[inline]
macro_rules! m_e {
    () => {};
}

#[inline] // { dg-error ".E0518." "" { target *-*-* } }
macro_rules! m {
    () => {};
}

fn main() {}


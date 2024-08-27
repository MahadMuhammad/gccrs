#![feature(rustc_attrs, transparent_unions)]

#[rustc_pub_transparent]
#[repr(transparent)]
union E<T: Copy> {
    value: T,
    uninit: (),
}

#[repr(transparent)]
#[rustc_pub_transparent]
struct S<T>(T);

#[rustc_pub_transparent] // { dg-error "" "" { target *-*-* } }
#[repr(C)]
struct S1 {
    A: u8,
}

#[rustc_pub_transparent] // { dg-error "" "" { target *-*-* } }
struct S2<T> {
    value: T,
}

fn main() {}


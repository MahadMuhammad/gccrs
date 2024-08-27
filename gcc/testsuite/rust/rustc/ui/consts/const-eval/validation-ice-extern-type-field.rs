#![feature(extern_types)]

extern {
    type Opaque;
}

struct ThinDst {
    x: u8,
    tail: Opaque,
}

const C1: &ThinDst = unsafe { std::mem::transmute(b"d".as_ptr()) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {}


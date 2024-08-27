use std::mem::offset_of;

struct S {
    v: u8,
    w: u16,
}


fn main() {
    let _: u8 = offset_of!(S, v); // { dg-error ".E0308." "" { target *-*-* } }
    let _: u16 = offset_of!(S, v); // { dg-error ".E0308." "" { target *-*-* } }
    let _: u32 = offset_of!(S, v); // { dg-error ".E0308." "" { target *-*-* } }
    let _: u64 = offset_of!(S, v); // { dg-error ".E0308." "" { target *-*-* } }
    let _: isize = offset_of!(S, v); // { dg-error ".E0308." "" { target *-*-* } }
    let _: usize = offset_of!(S, v);

    offset_of!(S, v) // { dg-error ".E0308." "" { target *-*-* } }
}


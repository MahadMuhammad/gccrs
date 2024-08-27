//@ compile-flags: --test

fn align_offset_weird_strides() {
    #[test]
// { dg-error "" "" { target *-*-* } .-1 }
    struct A5(u32, u8);
}


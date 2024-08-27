use std::mem::offset_of;

struct C<T> {
    v: T,
    w: T,
}

struct S {
    v: u8,
    w: u16,
}

impl S {
    fn v_offs() -> usize {
        offset_of!(Self, v)
    }
    fn v_offs_wrong_syntax() {
        offset_of!(Self, Self::v); // { dg-error "" "" { target *-*-* } }
        offset_of!(S, Self); // { dg-error ".E0609." "" { target *-*-* } }
    }
    fn offs_in_c() -> usize {
        offset_of!(C<Self>, w)
    }
    fn offs_in_c_colon() -> usize {
        offset_of!(C::<Self>, w)
    }
}

mod m {
    use std::mem::offset_of;
    fn off() {
        offset_of!(self::S, v); // { dg-error ".E0412." "" { target *-*-* } }
        offset_of!(super::S, v);
        offset_of!(crate::S, v);
    }
    impl super::n::T {
        fn v_offs_self() -> usize {
            offset_of!(Self, v) // { dg-error ".E0616." "" { target *-*-* } }
        }
    }
}

mod n {
    pub struct T { v: u8, }
}

fn main() {
    offset_of!(self::S, v);
    offset_of!(Self, v); // { dg-error ".E0411." "" { target *-*-* } }

    offset_of!(S, self); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!(S, v.self); // { dg-error ".E0609." "" { target *-*-* } }
}


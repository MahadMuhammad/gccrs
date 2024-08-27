//@ compile-flags: --crate-type lib
pub struct S {
    pub val: string::MyString,
}

pub fn test(s: S) {
    dbg!(s.cap) // { dg-error ".E0609." "" { target *-*-* } }
}

pub(crate) mod string {

    pub struct MyString {
        buf: MyVec,
    }

    struct MyVec {
        cap: usize,
    }
}


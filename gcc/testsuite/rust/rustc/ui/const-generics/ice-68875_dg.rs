//@ check-fail

struct DataWrapper<'a> {
    data: &'a [u8; Self::SIZE], // { dg-error "" "" { target *-*-* } }
}

impl DataWrapper<'_> {
    const SIZE: usize = 14;
}

pub fn main() {}


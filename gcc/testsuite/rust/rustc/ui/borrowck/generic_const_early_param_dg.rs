#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }

struct DataWrapper<'static> {
// { dg-error ".E0262." "" { target *-*-* } .-1 }
    data: &'a [u8; Self::SIZE],
// { dg-error ".E0261." "" { target *-*-* } .-1 }
}

impl DataWrapper<'a> {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
    const SIZE: usize = 14;
}

fn main(){}


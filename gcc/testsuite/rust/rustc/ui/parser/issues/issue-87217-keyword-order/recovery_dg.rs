// test for #115714

struct Misplaced;

impl Misplaced {
    unsafe const fn from_u32(val: u32) {}
// { dg-error "" "" { target *-*-* } .-1 }
    fn oof(self){}
}

struct Duplicated;

impl Duplicated {
    unsafe unsafe fn from_u32(val: u32) {}
// { dg-error "" "" { target *-*-* } .-1 }
    fn oof(self){}
}

fn main() {
    Misplaced.oof();
    Duplicated.oof();
}


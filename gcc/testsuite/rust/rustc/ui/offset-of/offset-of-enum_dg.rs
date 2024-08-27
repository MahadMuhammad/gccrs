#![feature(offset_of_enum)]

use std::mem::offset_of;

enum Alpha {
    One(u8),
    Two(u8),
}

fn main() {
    offset_of!(Alpha::One, 0); // { dg-error ".E0573." "" { target *-*-* } }
    offset_of!(Alpha, One); // { dg-error ".E0795." "" { target *-*-* } }
    offset_of!(Alpha, Two.0);
    offset_of!(Alpha, Two.1); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!(Alpha, Two.foo); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!(Alpha, NonExistent); // { dg-error ".E0599." "" { target *-*-* } }
    offset_of!(Beta, One); // { dg-error ".E0412." "" { target *-*-* } }
}


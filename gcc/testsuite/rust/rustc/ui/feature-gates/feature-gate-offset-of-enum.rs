use std::mem::offset_of;

enum Alpha {
    One(u8),
    Two(u8),
}

fn main() {
    offset_of!(Alpha::One, 0); // { dg-error ".E0573." "" { target *-*-* } }
    offset_of!(Alpha, One); // { dg-error ".E0795." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    offset_of!(Alpha, Two.0); // { dg-error ".E0658." "" { target *-*-* } }
}


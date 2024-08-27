#![feature(type_ascription)]

fn main() {
    type_ascribe!(2, n([u8; || 1]))
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}


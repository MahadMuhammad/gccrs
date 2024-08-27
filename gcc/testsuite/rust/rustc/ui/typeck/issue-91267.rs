#![feature(type_ascription)]

fn main() {
    type_ascribe!(0, u8<e<5>=e>)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}


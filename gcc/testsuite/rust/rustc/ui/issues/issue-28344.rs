use std::ops::BitXor;

fn main() {
    let x: u8 = BitXor::bitor(0 as u8, 0 as u8);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
// { dg-warning ".E0599." "" { target *-*-* } .-4 }

    let g = BitXor::bitor;
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
// { dg-warning ".E0599." "" { target *-*-* } .-4 }
}


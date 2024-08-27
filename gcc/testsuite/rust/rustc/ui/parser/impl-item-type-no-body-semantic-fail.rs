fn main() {}

struct X;

impl X {
    type Y;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    type Z: Ord;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
    type W: Ord where Self: Eq;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
    type W where Self: Eq;
// { dg-error ".E0592." "" { target *-*-* } .-1 }
// { dg-error ".E0592." "" { target *-*-* } .-2 }
// { dg-error ".E0592." "" { target *-*-* } .-3 }
// { dg-error ".E0592." "" { target *-*-* } .-4 }
}


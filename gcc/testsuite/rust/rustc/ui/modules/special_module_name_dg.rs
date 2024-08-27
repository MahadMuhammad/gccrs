mod lib;
// { dg-warning ".E0583." "" { target *-*-* } .-1 }
// { dg-error ".E0583." "" { target *-*-* } .-2 }
mod main;
// { dg-warning ".E0583." "" { target *-*-* } .-1 }
// { dg-error ".E0583." "" { target *-*-* } .-2 }

fn main() {}


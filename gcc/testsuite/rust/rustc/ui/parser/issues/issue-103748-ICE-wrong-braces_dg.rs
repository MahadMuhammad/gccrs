#![crate_type = "lib"]

struct Apple((Apple, Option(Banana ? Citron)));
// { dg-error ".E0072." "" { target *-*-* } .-1 }
// { dg-error ".E0072." "" { target *-*-* } .-2 }
// { dg-error ".E0072." "" { target *-*-* } .-3 }
// { dg-error ".E0072." "" { target *-*-* } .-4 }
// { dg-error ".E0072." "" { target *-*-* } .-5 }


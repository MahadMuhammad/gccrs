#![u=||{static d=||1;}]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

#![a={impl std::ops::Neg for i8 {}}]
// { dg-error ".E0601." "" { target *-*-* } .-1 }
// { dg-error ".E0601." "" { target *-*-* } .-2 }
// { dg-error ".E0601." "" { target *-*-* } .-3 }


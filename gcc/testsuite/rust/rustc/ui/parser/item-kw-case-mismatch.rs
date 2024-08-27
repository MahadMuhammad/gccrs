//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
#![allow(unused_imports)]

fn main() {}

Use std::ptr::read;  // { dg-error "" "" { target *-*-* } }
USE std::ptr::write; // { dg-error "" "" { target *-*-* } }

async Fn _a() {}
// { dg-error "" "" { target *-*-* } .-1 }

Fn _b() {}
// { dg-error "" "" { target *-*-* } .-1 }

aSYNC fN _c() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

Async fn _d() {}
// { dg-error "" "" { target *-*-* } .-1 }

CONST UNSAFE FN _e() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

unSAFE EXTern fn _f() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

EXTERN "C" FN _g() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }


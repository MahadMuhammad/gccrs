fn f1() -> impl Sized { & 2E } // { dg-error "" "" { target *-*-* } }
fn f2() -> impl Sized { && 2E } // { dg-error "" "" { target *-*-* } }
fn f3() -> impl Sized { &'a 2E } // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
fn f4() -> impl Sized { &'static 2E } // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
fn f5() -> impl Sized { *& 2E } // { dg-error "" "" { target *-*-* } }
fn f6() -> impl Sized { &'_ 2E } // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {}


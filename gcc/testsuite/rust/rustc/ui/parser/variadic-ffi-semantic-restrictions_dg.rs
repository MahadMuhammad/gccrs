#![feature(c_variadic)]
#![allow(anonymous_parameters)]

fn main() {}

fn f1_1(x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn f1_2(...) {}
// { dg-error "" "" { target *-*-* } .-1 }

extern "C" fn f2_1(x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }

extern "C" fn f2_2(...) {}
// { dg-error "" "" { target *-*-* } .-1 }

extern "C" fn f2_3(..., x: isize) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

extern "C" fn f3_1(x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }

extern "C" fn f3_2(...) {}
// { dg-error "" "" { target *-*-* } .-1 }

extern "C" fn f3_3(..., x: isize) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

const unsafe extern "C" fn f4_1(x: isize, ...) {}
// { dg-error ".E0493." "" { target *-*-* } .-1 }
// { dg-error ".E0493." "" { target *-*-* } .-2 }

const extern "C" fn f4_2(x: isize, ...) {}
// { dg-error ".E0493." "" { target *-*-* } .-1 }
// { dg-error ".E0493." "" { target *-*-* } .-2 }
// { dg-error ".E0493." "" { target *-*-* } .-3 }

const extern "C" fn f4_3(..., x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

extern "C" {
    fn e_f2(..., x: isize);
// { dg-error "" "" { target *-*-* } .-1 }
}

struct X;

impl X {
    fn i_f1(x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }
    fn i_f2(...) {}
// { dg-error "" "" { target *-*-* } .-1 }
    fn i_f3(..., x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    fn i_f4(..., x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    const fn i_f5(x: isize, ...) {}
// { dg-error ".E0493." "" { target *-*-* } .-1 }
// { dg-error ".E0493." "" { target *-*-* } .-2 }
// { dg-error ".E0493." "" { target *-*-* } .-3 }
}

trait T {
    fn t_f1(x: isize, ...) {}
// { dg-error "" "" { target *-*-* } .-1 }
    fn t_f2(x: isize, ...);
// { dg-error "" "" { target *-*-* } .-1 }
    fn t_f3(...) {}
// { dg-error "" "" { target *-*-* } .-1 }
    fn t_f4(...);
// { dg-error "" "" { target *-*-* } .-1 }
    fn t_f5(..., x: isize) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    fn t_f6(..., x: isize);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}


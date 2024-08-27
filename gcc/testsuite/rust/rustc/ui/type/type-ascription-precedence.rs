// Operator precedence of type ascription
// Type ascription has very high precedence, the same as operator `as`
#![feature(type_ascription)]

use std::ops::*;
struct S;
struct Z;

impl Add<Z> for S {
    type Output = S;
    fn add(self, _rhs: Z) -> S { panic!() }
}
impl Mul<Z> for S {
    type Output = S;
    fn mul(self, _rhs: Z) -> S { panic!() }
}
impl Neg for S {
    type Output = Z;
    fn neg(self) -> Z { panic!() }
}
impl Deref for S {
    type Target = Z;
    fn deref(&self) -> &Z { panic!() }
}

fn test1() {
    &S: &S; // { dg-error "" "" { target *-*-* } }
    (&S): &S;
    &(S: &S);
}

fn test2() {
    *(S: Z); // { dg-error "" "" { target *-*-* } }
}

fn test3() {
    -(S: Z); // { dg-error "" "" { target *-*-* } }
}

fn test4() {
    (S + Z): Z; // { dg-error "" "" { target *-*-* } }
}

fn test5() {
    (S * Z): Z; // { dg-error "" "" { target *-*-* } }
}

fn test6() {
    S .. S: S; // { dg-error "" "" { target *-*-* } }
}

fn test7() {
    (S .. S): S; // { dg-error "" "" { target *-*-* } }
}

fn main() {}


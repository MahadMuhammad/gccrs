//@ run-rustfix

#![feature(let_chains)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]

fn err_some(b: bool, x: Option<u32>) {
    if b && if let Some(x) = x {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn err_none(b: bool, x: Option<u32>) {
    if b && if let None = x {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn err_bool_1() {
    if true && if true { true } else { false };
// { dg-error "" "" { target *-*-* } .-1 }
}

fn err_bool_2() {
    if true && if false { true } else { false };
// { dg-error "" "" { target *-*-* } .-1 }
}

fn should_ok_1() {
    if true && if let x = 1 { true } else { true } {}
}

fn should_ok_2() {
    if true && if let 1 = 1 { true } else { true } {}
}

fn should_ok_3() {
    if true && if true { true } else { false } {}
}

fn should_ok_in_nested() {
    if true && if true { true } else { false } { true } else { false };
}

fn main() {}


// Evaluation of constants in refutable patterns goes through
// different compiler control-flow paths.

#![allow(unused_imports, warnings)]

use std::fmt;
use std::{i8, i16, i32, i64, isize};
use std::{u8, u16, u32, u64, usize};

const NEG_128: i8 = -128;
const NEG_NEG_128: i8 = -NEG_128; // { dg-error ".E0080." "" { target *-*-* } }

fn main() {
    match -128i8 {
        NEG_NEG_128 => println!("A"),
// { dg-error "" "" { target *-*-* } .-1 }
        _ => println!("B"),
    }
}


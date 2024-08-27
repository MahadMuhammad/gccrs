// run-rustifx
#![allow(unused)]
use std::sync::{Arc, Mutex};

pub struct Foo {
    a: Mutex<usize>,
    b: Arc<Mutex<usize>, // { help "" "" { target *-*-* } }
    c: Arc<Mutex<usize>>,
} // { dg-error "" "" { target *-*-* } }

fn main() {}


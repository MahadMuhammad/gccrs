//@ run-rustfix
#![allow(dead_code)]

struct Foo {
    v: Vec<u32>,
    h: std::collections::HashMap<i32, i32>,
}

impl Foo {
    fn bar(&self) {
        for _ in self.v { // { dg-error ".E0507." "" { target *-*-* } }
        }
        for _ in self.h { // { dg-error ".E0507." "" { target *-*-* } }
        }
    }
}

const LOADERS: &Vec<&'static u8> = &Vec::new();

pub fn break_code() -> Option<&'static u8> {
    for loader in *LOADERS { // { dg-error ".E0507." "" { target *-*-* } }
        return Some(loader);
    }
    None
}

fn main() {}


//@ compile-flags: --edition 2018

pub fn demo() -> Option<i32> {
    do yeet // { dg-error ".E0658." "" { target *-*-* } }
}

pub fn main() -> Result<(), String> {
    do yeet "hello"; // { dg-error ".E0658." "" { target *-*-* } }
}


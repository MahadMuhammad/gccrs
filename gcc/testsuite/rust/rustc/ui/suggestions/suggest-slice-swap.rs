//@ run-rustfix
#![allow(dead_code)]

fn swap(arr: &mut [u32; 2]) {
    std::mem::swap(&mut arr[0], &mut arr[1]);
// { dg-error ".E0499." "" { target *-*-* } .-1 }
}

fn main() {}


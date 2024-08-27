// Regression test for #64620

#![feature(coroutines)]

pub fn crash(arr: [usize; 1]) {
    yield arr[0]; // { dg-error ".E0627." "" { target *-*-* } }
// { dg-error ".E0627." "" { target *-*-* } .-1 }
}

fn main() {}


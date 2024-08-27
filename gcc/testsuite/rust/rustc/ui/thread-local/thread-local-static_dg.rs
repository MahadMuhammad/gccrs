// { dg-additional-options "-frust-edition=2018" }

#![feature(thread_local)]
#![feature(const_swap)]
#![allow(static_mut_refs)]

#[thread_local]
static mut STATIC_VAR_2: [u32; 8] = [4; 8];
const fn g(x: &mut [u32; 8]) {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    std::mem::swap(x, &mut STATIC_VAR_2)
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
}

fn main() {}


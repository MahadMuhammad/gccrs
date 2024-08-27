#![feature(const_for)]
#![feature(const_mut_refs)]

const _: () = {
    for _ in 0..5 {}
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
};

fn main() {}


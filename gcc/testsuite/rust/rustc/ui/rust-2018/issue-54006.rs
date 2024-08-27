// { dg-additional-options "-frust-edition=2018" }

#![no_std]
#![crate_type = "lib"]

use alloc::vec;
// { dg-error ".E0432." "" { target *-*-* } .-1 }

pub fn foo() {
    let mut xs = vec![];
    xs.push(0);
}


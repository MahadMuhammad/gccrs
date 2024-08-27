#![feature(core_intrinsics)]

extern crate core;
use core::intrinsics::mir::*; // { dg-error ".E0658." "" { target *-*-* } }

#[custom_mir(dialect = "built")] // { dg-error ".E0658." "" { target *-*-* } }
pub fn foo(_x: i32) -> i32 {
    mir! {
        {
            Return() // { dg-error ".E0658." "" { target *-*-* } }
        }
    }
}

fn main() {
    assert_eq!(2, foo(2));
}


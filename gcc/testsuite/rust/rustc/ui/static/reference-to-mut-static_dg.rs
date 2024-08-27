//@ revisions: e2021 e2024

// { dg-additional-options "-frust-edition=2021" }
//@ [e2024] compile-flags: --edition 2024 -Z unstable-options

#![deny(static_mut_refs)]

use std::ptr::{addr_of, addr_of_mut};

fn main() {
    static mut X: i32 = 1;

    static mut Y: i32 = 1;

    unsafe {
        let _y = &X;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        let _y = &mut X;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        let _z = addr_of_mut!(X);

        let _p = addr_of!(X);

        let ref _a = X;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        let (_b, _c) = (&X, &Y);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

        foo(&X);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        static mut Z: &[i32; 3] = &[0, 1, 2];

        let _ = Z.len();
        let _ = Z[0];
        let _ = format!("{:?}", Z);
    }
}

fn foo<'a>(_x: &'a i32) {}


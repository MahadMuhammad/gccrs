#![feature(stmt_expr_attributes)]

// Test that various placements of the inner attribute are parsed correctly,
// or not.

fn main() {
    let a = #![allow(warnings)] (1, 2);
// { dg-error "" "" { target *-*-* } .-1 }

    let b = (#![allow(warnings)] 1, 2);
// { dg-error "" "" { target *-*-* } .-1 }

    let c = {
        #![allow(warnings)]
        (#![allow(warnings)] 1, 2)
// { dg-error "" "" { target *-*-* } .-1 }
    };

    let d = {
        #![allow(warnings)]
        let e = (#![allow(warnings)] 1, 2);
// { dg-error "" "" { target *-*-* } .-1 }
        e
    };

    let e = [#![allow(warnings)] 1, 2];
// { dg-error "" "" { target *-*-* } .-1 }

    let f = [#![allow(warnings)] 1; 0];
// { dg-error "" "" { target *-*-* } .-1 }

    let g = match true { #![allow(warnings)] _ => {} };


    struct MyStruct { field: u8 }
    let h = MyStruct { #![allow(warnings)] field: 0 };
// { dg-error "" "" { target *-*-* } .-1 }
}


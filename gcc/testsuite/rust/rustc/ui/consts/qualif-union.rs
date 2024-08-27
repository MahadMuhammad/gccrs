// Checks that unions use type based qualification. Regression test for issue #90268.

use std::cell::Cell;
use std::mem::ManuallyDrop;

union U { i: u32, c: ManuallyDrop<Cell<u32>> }

const C1: ManuallyDrop<Cell<u32>> = {
    unsafe { U { c: ManuallyDrop::new(Cell::new(0)) }.c }
};

const C2: ManuallyDrop<Cell<u32>> = {
    unsafe { U { i : 0 }.c }
};

const C3: ManuallyDrop<Cell<u32>> = {
    let mut u = U { i: 0 };
    u.i = 1;
    unsafe { u.c }
};

const C4: U = U { i: 0 };

const C5: [U; 1] = [U {i : 0}; 1];

fn main() {
    // Interior mutability should prevent promotion.
    let _: &'static _ = &C1; // { dg-error ".E0716." "" { target *-*-* } }
    let _: &'static _ = &C2; // { dg-error ".E0716." "" { target *-*-* } }
    let _: &'static _ = &C3; // { dg-error ".E0716." "" { target *-*-* } }
    let _: &'static _ = &C4; // { dg-error ".E0716." "" { target *-*-* } }
    let _: &'static _ = &C5; // { dg-error ".E0716." "" { target *-*-* } }
}


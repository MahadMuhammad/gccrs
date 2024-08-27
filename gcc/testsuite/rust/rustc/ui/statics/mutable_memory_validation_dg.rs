//issue: rust-lang/rust#122548

// Strip out raw byte dumps to make comparison platform-independent:
//@ normalize-stderr-test: "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
//@ normalize-stderr-test: "([0-9a-f][0-9a-f] |╾─*A(LLOC)?[0-9]+(\+[a-z0-9]+)?(<imm>)?─*╼ )+ *│.*" -> "HEX_DUMP"

#![feature(const_mut_refs)]
#![feature(const_refs_to_static)]

use std::cell::UnsafeCell;

struct Meh {
    x: &'static UnsafeCell<i32>,
}

const MUH: Meh = Meh { x: unsafe { &mut *(&READONLY as *const _ as *mut _) } };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

static READONLY: i32 = 0;

pub fn main() {}


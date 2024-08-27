// ignore-tidy-linelength
// Strip out raw byte dumps to make comparison platform-independent:
//@ normalize-stderr-test: "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
//@ normalize-stderr-test: "([0-9a-f][0-9a-f] |╾─*ALLOC[0-9]+(\+[a-z0-9]+)?(<imm>)?─*╼ )+ *│.*" -> "HEX_DUMP"
#![allow(invalid_value)]

use std::mem;

#[repr(C)]
union MaybeUninit<T: Copy> {
    uninit: (),
    init: T,
}

const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

const NULL: &u16 = unsafe { mem::transmute(0usize) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }


// It is very important that we reject this: We do promote `&(4 * REF_AS_USIZE)`,
// but that would fail to compile; so we ended up breaking user code that would
// have worked fine had we not promoted.
const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }


const UNALIGNED_READ: () = unsafe {
    let x = &[0u8; 4];
    let ptr = x.as_ptr().cast::<u32>();
    ptr.read(); // { dg-error "" "" { target *-*-* } }
};


fn main() {}


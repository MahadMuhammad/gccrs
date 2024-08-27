//@ compile-flags: -Zunleash-the-miri-inside-of-you
//@ normalize-stderr-test: "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
//@ normalize-stderr-test: "([0-9a-f][0-9a-f] |╾─*ALLOC[0-9]+(\+[a-z0-9]+)?(<imm>)?─*╼ )+ *│.*" -> "HEX_DUMP"

#![allow(static_mut_refs)]
#![deny(const_eval_mutable_ptr_in_final_value)]
use std::cell::UnsafeCell;
use std::sync::atomic::*;

// # Plain `&mut` in the final value

// This requires walking nested statics.
static FOO: &&mut u32 = &&mut 42;
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
static OH_YES: &mut i32 = &mut 42;
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
static BAR: &mut () = &mut ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

struct Foo<T>(T);

static BOO: &mut Foo<()> = &mut Foo(());
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

const BLUNT: &mut i32 = &mut 42;
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

const SUBTLE: &mut i32 = unsafe { static mut STATIC: i32 = 0; &mut STATIC };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

// # Interior mutability

struct Meh {
    x: &'static UnsafeCell<i32>,
}
unsafe impl Sync for Meh {}
static MEH: Meh = Meh { x: &UnsafeCell::new(42) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
// Same with a const:
// the following will never be ok! no interior mut behind consts, because
// all allocs interned here will be marked immutable.
const MUH: Meh = Meh {
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
    x: &UnsafeCell::new(42),
};

struct Synced {
    x: UnsafeCell<i32>,
}
unsafe impl Sync for Synced {}

// Make sure we also catch this behind a type-erased `dyn Trait` reference.
const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

// # Check for mutable references to read-only memory

static READONLY: i32 = 0;
static mut MUT_TO_READONLY: &mut i32 = unsafe { &mut *(&READONLY as *const _ as *mut _) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

// # Check for consts pointing to mutable memory

static mut MUTABLE: i32 = 42;
const POINTS_TO_MUTABLE: &i32 = unsafe { &MUTABLE }; // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
static mut MUTABLE_REF: &mut i32 = &mut 42;
const POINTS_TO_MUTABLE2: &i32 = unsafe { &*MUTABLE_REF };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

const POINTS_TO_MUTABLE_INNER: *const i32 = &mut 42 as *mut _ as *const _;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

const POINTS_TO_MUTABLE_INNER2: *const i32 = &mut 42 as *const _;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

const INTERIOR_MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

struct SyncPtr<T> {
    x: *const T,
}
unsafe impl<T> Sync for SyncPtr<T> {}

// These pass the lifetime checks because of the "tail expression" / "outer scope" rule.
// (This relies on `SyncPtr` being a curly brace struct.)
// However, we intern the inner memory as read-only, so this must be rejected.
// (Also see `static-no-inner-mut` for similar tests on `static`.)
const RAW_SYNC: SyncPtr<AtomicI32> = SyncPtr { x: &AtomicI32::new(42) };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

const RAW_MUT_CAST: SyncPtr<i32> = SyncPtr { x: &mut 42 as *mut _ as *const _ };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

const RAW_MUT_COERCE: SyncPtr<i32> = SyncPtr { x: &mut 0 };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }


fn main() {
    unsafe {
        *MEH.x.get() = 99;
    }
    *OH_YES = 99; // { dg-error ".E0594." "" { target *-*-* } }
}


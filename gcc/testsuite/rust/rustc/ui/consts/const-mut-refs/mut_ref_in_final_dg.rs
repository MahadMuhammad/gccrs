#![feature(const_mut_refs)]

const NULL: *mut i32 = std::ptr::null_mut();
const A: *const i32 = &4;

// It could be made sound to allow it to compile,
// but we do not want to allow this to compile,
// as that would be an enormous footgun in oli-obk's opinion.
const B: *mut i32 = &mut 4; // { dg-error ".E0764." "" { target *-*-* } }

// Ok, no actual mutable allocation exists
const B2: Option<&mut i32> = None;

// Not ok, can't prove that no mutable allocation ends up in final value
const B3: Option<&mut i32> = Some(&mut 42); // { dg-error ".E0716." "" { target *-*-* } }

const fn helper(x: &mut i32) -> Option<&mut i32> { Some(x) }
const B4: Option<&mut i32> = helper(&mut 42); // { dg-error ".E0716." "" { target *-*-* } }

// Ok, because no references to mutable data exist here, since the `{}` moves
// its value and then takes a reference to that.
const C: *const i32 = &{
    let mut x = 42;
    x += 3;
    x
};

use std::cell::UnsafeCell;
struct NotAMutex<T>(UnsafeCell<T>);

unsafe impl<T> Sync for NotAMutex<T> {}

const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
// { dg-error ".E0716." "" { target *-*-* } .-1 }

static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
// { dg-error ".E0716." "" { target *-*-* } .-1 }

static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
// { dg-error ".E0716." "" { target *-*-* } .-1 }

// `BAR` works, because `&42` promotes immediately instead of relying on
// the enclosing scope rule.
const BAR: NotAMutex<&i32> = NotAMutex(UnsafeCell::new(&42));

struct SyncPtr<T> { x : *const T }
unsafe impl<T> Sync for SyncPtr<T> {}

// These pass the lifetime checks because of the "tail expression" / "outer scope" rule.
// (This relies on `SyncPtr` being a curly brace struct.)
// However, we intern the inner memory as read-only, so this must be rejected.
static RAW_MUT_CAST_S: SyncPtr<i32> = SyncPtr { x : &mut 42 as *mut _ as *const _ };
// { dg-error ".E0764." "" { target *-*-* } .-1 }
static RAW_MUT_COERCE_S: SyncPtr<i32> = SyncPtr { x: &mut 0 };
// { dg-error ".E0764." "" { target *-*-* } .-1 }
const RAW_MUT_CAST_C: SyncPtr<i32> = SyncPtr { x : &mut 42 as *mut _ as *const _ };
// { dg-error ".E0764." "" { target *-*-* } .-1 }
const RAW_MUT_COERCE_C: SyncPtr<i32> = SyncPtr { x: &mut 0 };
// { dg-error ".E0764." "" { target *-*-* } .-1 }

fn main() {
    println!("{}", unsafe { *A });
    unsafe { *B = 4 } // Bad news

    unsafe {
        **FOO.0.get() = 99;
        assert_eq!(**FOO.0.get(), 99);
    }
}


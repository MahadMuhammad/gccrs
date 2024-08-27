// This test checks that calling `mem::{uninitialized,zeroed}` with certain types results
// in a lint.
#![allow(deprecated)]
#![deny(invalid_value)]
#![feature(never_type, rustc_attrs)]

use std::mem::{self, MaybeUninit};
use std::ptr::NonNull;
use std::num::NonZero;

enum Void {}

struct Ref(&'static i32);
struct RefPair((&'static i32, i32));

struct Wrap<T> { wrapped: T }
enum WrapEnum<T> { Wrapped(T) }

#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(128)]
#[repr(transparent)]
pub(crate) struct NonBig(u64);

/// A two-variant enum, thus needs a tag and may not remain uninitialized.
enum Fruit {
    Apple,
    Banana,
}

/// Looks like two variants but really only has one.
enum OneFruit {
    Apple(!),
    Banana,
}

enum OneFruitNonZero {
    Apple(!),
    Banana(NonZero<u32>),
}

enum TwoUninhabited {
    A(!),
    B(Void),
}

#[rustc_layout_scalar_valid_range_start(254)]
#[rustc_layout_scalar_valid_range_end(1)]
pub(crate) struct WrapAroundRange(u8);

#[allow(unused)]
fn generic<T: 'static>() {
    unsafe {
        let _val: &'static T = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: &'static T = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Wrap<&'static T> = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: Wrap<&'static T> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }
    }
}

fn main() {
    unsafe {
        // Things that cannot even be zero.
        let _val: ! = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: ! = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: (i32, !) = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: (i32, !) = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Void = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: Void = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: &'static i32 = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: &'static i32 = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Ref = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: Ref = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: fn() = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: fn() = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Wrap<fn()> = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: Wrap<fn()> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: WrapEnum<fn()> = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: WrapEnum<fn()> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Wrap<(RefPair, i32)> = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: Wrap<(RefPair, i32)> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: NonNull<i32> = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: NonNull<i32> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: (NonZero<u32>, i32) = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: (NonZero<u32>, i32) = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: *const dyn Send = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: *const dyn Send = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: [fn(); 2] = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: [fn(); 2] = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: TwoUninhabited = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: TwoUninhabited = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: OneFruitNonZero = mem::zeroed(); // { dg-error "" "" { target *-*-* } }
        let _val: OneFruitNonZero = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        // Things that can be zero, but not uninit.
        let _val: bool = mem::zeroed();
        let _val: bool = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Wrap<char> = mem::zeroed();
        let _val: Wrap<char> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: NonBig = mem::zeroed();
        let _val: NonBig = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: Fruit = mem::zeroed();
        let _val: Fruit = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: [bool; 2] = mem::zeroed();
        let _val: [bool; 2] = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: i32 = mem::zeroed();
        let _val: i32 = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: f32 = mem::zeroed();
        let _val: f32 = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: *const () = mem::zeroed();
        let _val: *const () = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: *const [()] = mem::zeroed();
        let _val: *const [()] = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        let _val: WrapAroundRange = mem::zeroed();
        let _val: WrapAroundRange = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        // Things where 0 is okay due to rustc implementation details,
        // but that are not guaranteed to keep working.
        let _val: Result<i32, i32> = mem::zeroed();
        let _val: Result<i32, i32> = mem::uninitialized(); // { dg-error "" "" { target *-*-* } }

        // Some things that happen to be UB-free due to rustc implementation details,
        // but are not guaranteed to keep working.
        let _val: OneFruit = mem::zeroed();
        let _val: OneFruit = mem::uninitialized();

        // Transmute-from-0
        let _val: &'static i32 = mem::transmute(0usize); // { dg-error "" "" { target *-*-* } }
        let _val: &'static [i32] = mem::transmute((0usize, 0usize)); // { dg-error "" "" { target *-*-* } }
        let _val: NonZero<u32> = mem::transmute(0); // { dg-error "" "" { target *-*-* } }

        // `MaybeUninit` cases
        let _val: NonNull<i32> = MaybeUninit::zeroed().assume_init(); // { dg-error "" "" { target *-*-* } }
        let _val: NonNull<i32> = MaybeUninit::uninit().assume_init(); // { dg-error "" "" { target *-*-* } }
        let _val: bool = MaybeUninit::uninit().assume_init(); // { dg-error "" "" { target *-*-* } }

        // Some more types that should work just fine.
        let _val: Option<&'static i32> = mem::zeroed();
        let _val: Option<fn()> = mem::zeroed();
        let _val: MaybeUninit<&'static i32> = mem::zeroed();
        let _val: bool = MaybeUninit::zeroed().assume_init();
        let _val: [bool; 0] = MaybeUninit::uninit().assume_init();
        let _val: [!; 0] = MaybeUninit::zeroed().assume_init();
    }
}


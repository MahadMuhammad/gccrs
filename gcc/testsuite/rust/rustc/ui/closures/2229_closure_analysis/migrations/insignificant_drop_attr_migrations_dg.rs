//@ run-rustfix

#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }
#![feature(rustc_attrs)]
#![allow(unused)]

use std::sync::Mutex;

    #[rustc_insignificant_dtor]
struct InsignificantDropPoint {
    x: i32,
    y: Mutex<i32>,
}

impl Drop for InsignificantDropPoint {
    fn drop(&mut self) {}
}

struct SigDrop;

impl Drop for SigDrop {
    fn drop(&mut self) {}
}

#[rustc_insignificant_dtor]
struct GenericStruct<T>(T, T);

impl<T> Drop for GenericStruct<T> {
    fn drop(&mut self) {}
}

struct Wrapper<T>(GenericStruct<T>, i32);

// `SigDrop` implements drop and therefore needs to be migrated.
fn significant_drop_needs_migration() {
    let t = (SigDrop {}, SigDrop {});

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

// Even if a type implements an insignificant drop, if it's
// elements have a significant drop then the overall type is
// consdered to have an significant drop. Since the elements
// of `GenericStruct` implement drop, migration is required.
fn generic_struct_with_significant_drop_needs_migration() {
    let t = Wrapper(GenericStruct(SigDrop {}, SigDrop {}), 5);

    // move is used to force i32 to be copied instead of being a ref
    let c = move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.1;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

fn main() {
    significant_drop_needs_migration();
    generic_struct_with_significant_drop_needs_migration();
}


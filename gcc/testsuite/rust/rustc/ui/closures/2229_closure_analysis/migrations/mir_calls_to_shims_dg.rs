//@ run-rustfix
//@ needs-unwind

#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }
#![feature(fn_traits)]
#![feature(never_type)]

use std::panic;

fn foo_diverges() -> ! {
    panic!()
}

fn assert_panics<F>(f: F)
where
    F: FnOnce(),
{
    let f = panic::AssertUnwindSafe(f);
    let result = panic::catch_unwind(move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
        f.0()
// { dg-note "" "" { target *-*-* } .-1 }
    });
    if let Ok(..) = result {
        panic!("diverging function returned");
    }
}

fn test_fn_ptr_panic<T>(mut t: T)
where
    T: Fn() -> !,
{
    let as_fn = <T as Fn<()>>::call;
    assert_panics(|| as_fn(&t, ()));
    let as_fn_mut = <T as FnMut<()>>::call_mut;
    assert_panics(|| as_fn_mut(&mut t, ()));
    let as_fn_once = <T as FnOnce<()>>::call_once;
    assert_panics(|| as_fn_once(t, ()));
}

fn main() {
    test_fn_ptr_panic(foo_diverges);
    test_fn_ptr_panic(foo_diverges as fn() -> !);
}


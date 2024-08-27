#![feature(const_eval_select)]
#![feature(core_intrinsics)]

use std::intrinsics::const_eval_select;

const fn not_fn_items() {
    const_eval_select((), || {}, || {});
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    const_eval_select((), 42, 0xDEADBEEF);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
}

const fn foo(n: i32) -> i32 {
    n
}

fn bar(n: i32) -> bool {
    assert_eq!(n, 0, "{} must be equal to {}", n, 0);
    n == 0
}

fn baz(n: bool) -> i32 {
    assert!(n, "{} must be true", n);
    n as i32
}

const fn return_ty_mismatch() {
    const_eval_select((1,), foo, bar);
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

const fn args_ty_mismatch() {
    const_eval_select((true,), foo, baz);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
}

const fn non_const_fn() {
    const_eval_select((1,), bar, bar);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


//@ revisions: e2021 e2024
// { dg-additional-options "-frust-edition= 2021" }
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options

use std::env;

#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn unsafe_fn() {
    env::set_var("FOO", "BAR");
// { dg-error "" "" { target *-*-* } .-1 }
    env::remove_var("FOO");
// { dg-error "" "" { target *-*-* } .-1 }
    if false {
        unsafe_fn();
// { dg-error "" "" { target *-*-* } .-1 }
    }
}
fn safe_fn() {}

#[deny(unused_unsafe)]
fn main() {
    env::set_var("FOO", "BAR");
// { dg-error "" "" { target *-*-* } .-1 }
    env::remove_var("FOO");
// { dg-error "" "" { target *-*-* } .-1 }

    unsafe {
        env::set_var("FOO", "BAR");
        env::remove_var("FOO");
    }

    unsafe_fn();
// { dg-error "" "" { target *-*-* } .-1 }

    unsafe {
// { dg-error "" "" { target *-*-* } .-1 }
        safe_fn();
    }
}


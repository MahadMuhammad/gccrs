//@ run-rustfix

#![deny(deprecated_safe_2024)]

use std::env;

#[deny(unused_unsafe)]
fn main() {
    env::set_var("FOO", "BAR");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    env::remove_var("FOO");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    unsafe {
        env::set_var("FOO", "BAR");
        env::remove_var("FOO");
    }
}


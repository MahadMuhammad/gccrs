//@ run-rustfix
//@ rustfix-only-machine-applicable
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]

fn foo() -> i32 {
    {2} + {2} // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn bar() -> i32 {
    {2} + 2 // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn zul() -> u32 {
    let foo = 3;
    { 42 } + foo; // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    32
}

fn baz() -> i32 {
    { 3 } * 3 // { dg-error ".E0614." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn moo(x: u32) -> bool {
    match x {
        _ => 1,
    } > 0 // { dg-error "" "" { target *-*-* } }
}

fn qux() -> u32 {
    {2} - 2 // { dg-error ".E0600." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn space_cadet() -> bool {
    { true } | { true } // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn revenge_from_mars() -> bool {
    { true } && { true } // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn attack_from_mars() -> bool {
    { true } || { true } // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

// This gets corrected by adding a semicolon, instead of parens.
// It's placed here to help keep track of the way this diagnostic
// needs to interact with type checking to avoid MachineApplicable
// suggestions that actually break stuff.
//
// If you're wondering what happens if that `foo()` is a `true` like
// all the ones above use? Nothing. It makes neither suggestion in
// that case.
fn asteroids() -> impl FnOnce() -> bool {
    { foo() } || { true } // { dg-error ".E0308." "" { target *-*-* } }
}

// https://github.com/rust-lang/rust/issues/105179
fn r#match() -> i32 {
    match () { () => 1 } + match () { () => 1 } // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

// https://github.com/rust-lang/rust/issues/102171
fn r#unsafe() -> i32 {
    unsafe { 1 } + unsafe { 1 } // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}


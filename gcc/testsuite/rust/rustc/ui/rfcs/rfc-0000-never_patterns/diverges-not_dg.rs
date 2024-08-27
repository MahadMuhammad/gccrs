#![feature(never_patterns)]
#![feature(let_chains)]
#![allow(incomplete_features)]
#![deny(unreachable_patterns)]

fn main() {}

enum Void {}

// Contrast with `./diverges.rs`: merely having an empty type around isn't enough to diverge.

fn wild_void(_: Void) -> u32 {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn wild_let() -> u32 {
    let ptr: *const Void = std::ptr::null();
    unsafe {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _ = *ptr;
    }
}

fn wild_match() -> u32 {
    let ptr: *const Void = std::ptr::null();
    unsafe {
        match *ptr {
            _ => {} // { dg-error ".E0308." "" { target *-*-* } }
        }
    }
}

fn binding_void(_x: Void) -> u32 {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn binding_let() -> u32 {
    let ptr: *const Void = std::ptr::null();
    unsafe {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _x = *ptr;
    }
}

fn binding_match() -> u32 {
    let ptr: *const Void = std::ptr::null();
    unsafe {
        match *ptr {
            _x => {} // { dg-error ".E0308." "" { target *-*-* } }
        }
    }
}

// Don't confuse this with a `let !` statement.
fn let_chain(x: Void) -> u32 {
    if let true = true && let ! = x {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}


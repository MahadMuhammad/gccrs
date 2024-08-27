//@ only-x86_64
use std::sync::atomic::{AtomicPtr, Ordering};

fn main() {
    let ptr = &mut 5;
    let other_ptr = &mut 10;
    let x = AtomicPtr::new(ptr);

    // Allowed load ordering modes
    let _ = x.load(Ordering::Acquire);
    let _ = x.load(Ordering::SeqCst);
    let _ = x.load(Ordering::Relaxed);

    // Disallowed load ordering modes
    let _ = x.load(Ordering::Release);
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x.load(Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // Allowed store ordering modes
    x.store(other_ptr, Ordering::Release);
    x.store(other_ptr, Ordering::SeqCst);
    x.store(other_ptr, Ordering::Relaxed);

    // Disallowed store ordering modes
    x.store(other_ptr, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(other_ptr, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }
}


// FIXME: add support for `// only-atomic` to compiletest/header.rs
//@ only-x86_64
use std::sync::atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, Ordering};

fn main() {
    // `AtomicI8` test cases
    let x = AtomicI8::new(0);

    // Allowed load ordering modes
    let _ = x.load(Ordering::Acquire);
    let _ = x.load(Ordering::SeqCst);
    let _ = x.load(Ordering::Relaxed);

    // Allowed store ordering modes
    x.store(1, Ordering::Release);
    x.store(1, Ordering::SeqCst);
    x.store(1, Ordering::Relaxed);

    // Disallowed load ordering modes
    let _ = x.load(Ordering::Release);
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x.load(Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // Disallowed store ordering modes
    x.store(1, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(1, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // `AtomicI16` test cases
    let x = AtomicI16::new(0);

    // Allowed load ordering modes
    let _ = x.load(Ordering::Acquire);
    let _ = x.load(Ordering::SeqCst);
    let _ = x.load(Ordering::Relaxed);

    // Allowed store ordering modes
    x.store(1, Ordering::Release);
    x.store(1, Ordering::SeqCst);
    x.store(1, Ordering::Relaxed);

    // Disallowed load ordering modes
    let _ = x.load(Ordering::Release);
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x.load(Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // Disallowed store ordering modes
    x.store(1, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(1, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // `AtomicI32` test cases
    let x = AtomicI32::new(0);

    // Allowed load ordering modes
    let _ = x.load(Ordering::Acquire);
    let _ = x.load(Ordering::SeqCst);
    let _ = x.load(Ordering::Relaxed);

    // Allowed store ordering modes
    x.store(1, Ordering::Release);
    x.store(1, Ordering::SeqCst);
    x.store(1, Ordering::Relaxed);

    // Disallowed load ordering modes
    let _ = x.load(Ordering::Release);
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x.load(Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // Disallowed store ordering modes
    x.store(1, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(1, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // `AtomicI64` test cases
    let x = AtomicI64::new(0);

    // Allowed load ordering modes
    let _ = x.load(Ordering::Acquire);
    let _ = x.load(Ordering::SeqCst);
    let _ = x.load(Ordering::Relaxed);

    // Allowed store ordering modes
    x.store(1, Ordering::Release);
    x.store(1, Ordering::SeqCst);
    x.store(1, Ordering::Relaxed);

    // Disallowed load ordering modes
    let _ = x.load(Ordering::Release);
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x.load(Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // Disallowed store ordering modes
    x.store(1, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(1, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // `AtomicIsize` test cases
    let x = AtomicIsize::new(0);

    // Allowed load ordering modes
    let _ = x.load(Ordering::Acquire);
    let _ = x.load(Ordering::SeqCst);
    let _ = x.load(Ordering::Relaxed);

    // Allowed store ordering modes
    x.store(1, Ordering::Release);
    x.store(1, Ordering::SeqCst);
    x.store(1, Ordering::Relaxed);

    // Disallowed load ordering modes
    let _ = x.load(Ordering::Release);
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = x.load(Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }

    // Disallowed store ordering modes
    x.store(1, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(1, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }
}


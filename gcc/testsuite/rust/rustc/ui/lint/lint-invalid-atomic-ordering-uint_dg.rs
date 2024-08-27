//@ only-x86_64
use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};

fn main() {
    // `AtomicU8` test cases
    let x = AtomicU8::new(0);

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

    // `AtomicU16` test cases
    let x = AtomicU16::new(0);

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

    // `AtomicU32` test cases
    let x = AtomicU32::new(0);

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

    // `AtomicU64` test cases
    let x = AtomicU64::new(0);

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

    // `AtomicUsize` test cases
    let x = AtomicUsize::new(0);

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


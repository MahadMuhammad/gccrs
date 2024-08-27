//@ only-x86_64
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let x = AtomicBool::new(true);

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
    x.store(false, Ordering::Release);
    x.store(false, Ordering::SeqCst);
    x.store(false, Ordering::Relaxed);

    // Disallowed store ordering modes
    x.store(false, Ordering::Acquire);
// { dg-error "" "" { target *-*-* } .-1 }
    x.store(false, Ordering::AcqRel);
// { dg-error "" "" { target *-*-* } .-1 }
}


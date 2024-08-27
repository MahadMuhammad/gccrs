//@ only-x86_64
use std::sync::atomic::{compiler_fence, fence, Ordering};

fn main() {
    // Allowed ordering modes
    fence(Ordering::Acquire);
    fence(Ordering::Release);
    fence(Ordering::AcqRel);
    fence(Ordering::SeqCst);

    compiler_fence(Ordering::Acquire);
    compiler_fence(Ordering::Release);
    compiler_fence(Ordering::AcqRel);
    compiler_fence(Ordering::SeqCst);

    // Disallowed ordering modes
    fence(Ordering::Relaxed);
// { dg-error "" "" { target *-*-* } .-1 }
    compiler_fence(Ordering::Relaxed);
// { dg-error "" "" { target *-*-* } .-1 }
}


// Check that never patterns require the feature gate.
use std::ptr::NonNull;

enum Void {}

fn main() {
    let res: Result<u32, Void> = Ok(0);
    let (Ok(_x) | Err(&!)) = res.as_ref();
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    unsafe {
        let ptr: *const Void = NonNull::dangling().as_ptr();
        match *ptr {
            !
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        }
        // Check that the gate operates even behind `cfg`.
        #[cfg(FALSE)]
        match *ptr {
            !
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        }
        #[cfg(FALSE)]
        match *ptr {
            ! => {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        }
    }

    // Correctly gate match arms with no body.
    match Some(0) {
        None => {}
        Some(_),
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match Some(0) {
        None => {}
        Some(_)
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match Some(0) {
        _ => {}
        Some(_) if false,
// { dg-error "" "" { target *-*-* } .-1 }
        Some(_) if false
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match res {
        Ok(_) => {}
        Err(!),
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    }
    match res {
        Err(!) if false,
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
        _ => {}
    }

    // Check that the gate operates even behind `cfg`.
    match Some(0) {
        None => {}
        #[cfg(FALSE)]
        Some(_)
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match Some(0) {
        _ => {}
        #[cfg(FALSE)]
        Some(_) if false
// { dg-error "" "" { target *-*-* } .-1 }
    }
}


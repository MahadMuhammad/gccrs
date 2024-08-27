//@ revisions: e2015 e2024
//@[e2015] check-pass
//@[e2024] check-fail
// { dg-additional-options "-frust-edition=2024" }
//@[e2024] compile-flags: -Zunstable-options

use std::{marker, mem, ptr};

fn main() {}

fn _zero() {
    if false {
        unsafe { mem::zeroed() }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
    } else {
        return;
    };

    // no ; -> type is inferred without fallback
    if true { unsafe { mem::zeroed() } } else { return }
}

fn _trans() {
    if false {
        unsafe {
            struct Zst;
            core::mem::transmute(Zst)
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
        }
    } else {
        return;
    };
}

fn _union() {
    if false {
        union Union<T: Copy> {
            a: (),
            b: T,
        }

        unsafe { Union { a: () }.b }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
    } else {
        return;
    };
}

fn _deref() {
    if false {
        unsafe { *ptr::from_ref(&()).cast() }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
    } else {
        return;
    };
}

fn _only_generics() {
    if false {
        unsafe fn internally_create<T>(_: Option<T>) {
            unsafe {
                let _ = mem::zeroed::<T>();
            }
        }

        // We need the option (and unwrap later) to call a function in a way,
        // which makes it affected by the fallback, but without having it return anything
        let x = None;

        unsafe { internally_create(x) }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }

        x.unwrap()
    } else {
        return;
    };
}

fn _stored_function() {
    if false {
        let zeroed = mem::zeroed;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }

        unsafe { zeroed() }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
    } else {
        return;
    };
}

fn _only_generics_stored_function() {
    if false {
        unsafe fn internally_create<T>(_: Option<T>) {
            unsafe {
                let _ = mem::zeroed::<T>();
            }
        }

        let x = None;
        let f = internally_create;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }

        unsafe { f(x) }

        x.unwrap()
    } else {
        return;
    };
}

fn _method() {
    struct S<T>(marker::PhantomData<T>);

    impl<T> S<T> {
        #[allow(unused)] // FIXME: the unused lint is probably incorrect here
        unsafe fn create_out_of_thin_air(&self) -> T {
            todo!()
        }
    }

    if false {
        unsafe {
            S(marker::PhantomData).create_out_of_thin_air()
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
        }
    } else {
        return;
    };
}

// Minimization of the famous `objc` crate issue
fn _objc() {
    pub unsafe fn send_message<R>() -> Result<R, ()> {
        Ok(unsafe { core::mem::zeroed() })
    }

    macro_rules! msg_send {
        () => {
            match send_message::<_ /* ?0 */>() {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
                Ok(x) => x,
                Err(_) => loop {},
            }
        };
    }

    unsafe {
        msg_send!();
    }
}


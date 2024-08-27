//@ run-rustfix
#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }

use std::thread;

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

/* Test Send Trait Migration */
struct SendPointer(*mut i32);
unsafe impl Send for SendPointer {}

fn test_send_trait() {
    let mut f = 10;
    let fptr = SendPointer(&mut f as *mut i32);
    thread::spawn(move || unsafe {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
        *fptr.0 = 20;
// { dg-note "" "" { target *-*-* } .-1 }
    }).join().unwrap();
}

/* Test Sync Trait Migration */
struct CustomInt(*mut i32);
struct SyncPointer(CustomInt);
unsafe impl Sync for SyncPointer {}
unsafe impl Send for CustomInt {}

fn test_sync_trait() {
    let mut f = 10;
    let f = CustomInt(&mut f as *mut i32);
    let fptr = SyncPointer(f);
    thread::spawn(move || unsafe {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
        *fptr.0.0 = 20;
// { dg-note "" "" { target *-*-* } .-1 }
    }).join().unwrap();
}

/* Test Clone Trait Migration */
struct S(#[allow(dead_code)] Foo);
struct T(i32);

struct U(#[allow(dead_code)] S, T);

impl Clone for U {
    fn clone(&self) -> Self {
        U(S(Foo(0)), T(0))
    }
}

fn test_clone_trait() {
    let f = U(S(Foo(0)), T(0));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
        let f_1 = f.1;
// { dg-note "" "" { target *-*-* } .-1 }
        println!("{:?}", f_1.0);
    };

    let c_clone = c.clone();

    c_clone();
}
// { dg-note "" "" { target *-*-* } .-1 }

fn main() {
    test_send_trait();
    test_sync_trait();
    test_clone_trait();
}


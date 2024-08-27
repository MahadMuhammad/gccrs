//@ run-rustfix
#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }

use std::thread;

#[derive(Debug)]
struct Foo(String);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

impl Foo {
    fn from(s: &str) -> Self {
        Self(String::from(s))
    }
}

struct S(#[allow(dead_code)] Foo);

#[derive(Clone)]
struct T(#[allow(dead_code)] i32);

struct U(S, T);

impl Clone for U {
    fn clone(&self) -> Self {
        U(S(Foo::from("Hello World")), T(0))
    }
}

fn test_multi_issues() {
    let f1 = U(S(Foo::from("foo")), T(0));
    let f2 = U(S(Foo::from("bar")), T(0));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
        let _f_1 = f1.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _f_2 = f2.1;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    let c_clone = c.clone();

    c_clone();
}
// { dg-note "" "" { target *-*-* } .-1 }

fn test_capturing_all_disjoint_fields_individually() {
    let f1 = U(S(Foo::from("foo")), T(0));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
        let _f_1 = f1.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _f_2 = f1.1;
    };

    let c_clone = c.clone();

    c_clone();
}

struct U1(S, T, S);

impl Clone for U1 {
    fn clone(&self) -> Self {
        U1(S(Foo::from("foo")), T(0), S(Foo::from("bar")))
    }
}

fn test_capturing_several_disjoint_fields_individually_1() {
    let f1 = U1(S(Foo::from("foo")), T(0), S(Foo::from("bar")));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
        let _f_0 = f1.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _f_2 = f1.2;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    let c_clone = c.clone();

    c_clone();
}

fn test_capturing_several_disjoint_fields_individually_2() {
    let f1 = U1(S(Foo::from("foo")), T(0), S(Foo::from("bar")));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
        let _f_0 = f1.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _f_1 = f1.1;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    let c_clone = c.clone();

    c_clone();
}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

struct SendPointer(*mut i32);
unsafe impl Send for SendPointer {}

struct CustomInt(*mut i32);
struct SyncPointer(CustomInt);
unsafe impl Sync for SyncPointer {}
unsafe impl Send for CustomInt {}

fn test_multi_traits_issues() {
    let mut f1 = 10;
    let f1 = CustomInt(&mut f1 as *mut i32);
    let fptr1 = SyncPointer(f1);

    let mut f2 = 10;
    let fptr2 = SendPointer(&mut f2 as *mut i32);
    thread::spawn(move || unsafe {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
        *fptr1.0.0 = 20;
// { dg-note "" "" { target *-*-* } .-1 }
        *fptr2.0 = 20;
// { dg-note "" "" { target *-*-* } .-1 }
    }).join().unwrap();
}

fn main() {
    test_multi_issues();
    test_capturing_all_disjoint_fields_individually();
    test_capturing_several_disjoint_fields_individually_1();
    test_capturing_several_disjoint_fields_individually_2();
    test_multi_traits_issues();
}


#![allow(unused, dead_code)]

static mut FOO: u64 = 0;

fn static_mod() {
    unsafe {static BAR: u64 = FOO;}
// { dg-error ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
// { dg-note ".E0133." "" { target *-*-* } .-4 }
}

unsafe fn unsafe_call() {}
fn foo() {
    unsafe {
// { dg-note "" "" { target *-*-* } .-1 }
        fn bar() {
            unsafe_call();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
        }
    }
}

fn main() {}


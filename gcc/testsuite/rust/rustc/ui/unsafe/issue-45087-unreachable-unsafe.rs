// Verify that unreachable code undergoes unsafety checks.

fn main() {
    return;
    *(1 as *mut u32) = 42;
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

fn panic() -> ! {
    panic!();
}

fn f(a: *mut u32) {
    panic();
    *a = 1;
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

enum Void {}

fn uninhabited() -> Void {
    panic!();
}

fn g(b: *mut u32) {
    uninhabited();
    *b = 1;
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}


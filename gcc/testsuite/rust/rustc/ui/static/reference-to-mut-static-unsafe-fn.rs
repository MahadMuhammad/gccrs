//@ compile-flags: --edition 2024 -Z unstable-options

fn main() {}

unsafe fn _foo() {
    unsafe {
        static mut X: i32 = 1;
        static mut Y: i32 = 1;

        let _y = &X;
// { dg-error ".E0796." "" { target *-*-* } .-1 }

        let ref _a = X;
// { dg-error ".E0796." "" { target *-*-* } .-1 }

        let ref mut _a = X;
// { dg-error ".E0796." "" { target *-*-* } .-1 }

        let (_b, _c) = (&X, &mut Y);
// { dg-error ".E0796." "" { target *-*-* } .-1 }
// { dg-error ".E0796." "" { target *-*-* } .-2 }

        foo(&X);
// { dg-error ".E0796." "" { target *-*-* } .-1 }
    }
}

fn foo<'a>(_x: &'a i32) {}


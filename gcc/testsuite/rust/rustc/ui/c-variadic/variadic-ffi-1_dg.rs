//@ needs-llvm-components: x86
//@ compile-flags: --target=i686-pc-windows-msvc --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang="sized"]
trait Sized { }

extern "stdcall" {
    fn printf(_: *const u8, ...);
// { dg-error ".E0045." "" { target *-*-* } .-1 }
    // like C, cdecl, win64, sysv64 or efiapi
}

extern "C" {
    fn foo(f: isize, x: u8, ...);
}

extern "C" fn bar(f: isize, x: u8) {}

fn main() {
    unsafe {
        foo(); // { dg-error ".E0060." "" { target *-*-* } }
        foo(1); // { dg-error ".E0060." "" { target *-*-* } }

        let x: unsafe extern "C" fn(f: isize, x: u8) = foo; // { dg-error ".E0308." "" { target *-*-* } }
        let y: extern "C" fn(f: isize, x: u8, ...) = bar; // { dg-error ".E0308." "" { target *-*-* } }

        foo(1, 2, 3f32); // { dg-error ".E0617." "" { target *-*-* } }
        foo(1, 2, true); // { dg-error ".E0617." "" { target *-*-* } }
        foo(1, 2, 1i8); // { dg-error ".E0617." "" { target *-*-* } }
        foo(1, 2, 1u8); // { dg-error ".E0617." "" { target *-*-* } }
        foo(1, 2, 1i16); // { dg-error ".E0617." "" { target *-*-* } }
        foo(1, 2, 1u16); // { dg-error ".E0617." "" { target *-*-* } }
    }
}


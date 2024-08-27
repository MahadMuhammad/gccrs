//@ needs-asm-support
#![feature(naked_functions)]
#![feature(fn_align)]
#![crate_type = "lib"]
use std::arch::asm;

#[repr(C)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
#[naked]
extern "C" fn example1() {
// { dg-note "" "" { target *-*-* } .-1 }
    unsafe { asm!("", options(noreturn)) }
}

#[repr(transparent)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
#[naked]
extern "C" fn example2() {
// { dg-note "" "" { target *-*-* } .-1 }
    unsafe { asm!("", options(noreturn)) }
}

#[repr(align(16), C)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
#[naked]
extern "C" fn example3() {
// { dg-note "" "" { target *-*-* } .-1 }
    unsafe { asm!("", options(noreturn)) }
}

// note: two errors because of packed and C
#[repr(C, packed)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
// { dg-error ".E0517." "" { target *-*-* } .-2 }
#[naked]
extern "C" fn example4() {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    unsafe { asm!("", options(noreturn)) }
}

#[repr(u8)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
#[naked]
extern "C" fn example5() {
// { dg-note "" "" { target *-*-* } .-1 }
    unsafe { asm!("", options(noreturn)) }
}


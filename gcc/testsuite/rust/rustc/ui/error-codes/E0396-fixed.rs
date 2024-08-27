#![feature(const_mut_refs)]

const REG_ADDR: *mut u8 = 0x5f3759df as *mut u8;

const VALUE: u8 = unsafe { *REG_ADDR };
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {
}


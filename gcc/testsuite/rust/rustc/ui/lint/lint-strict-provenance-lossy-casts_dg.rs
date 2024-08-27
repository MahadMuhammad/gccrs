#![feature(strict_provenance)]
#![deny(lossy_provenance_casts)]

fn main() {
    let x: u8 = 37;
    let addr: usize = &x as *const u8 as usize;
// { dg-error "" "" { target *-*-* } .-1 }

    let addr_32bit = &x as *const u8 as u32;
// { dg-error "" "" { target *-*-* } .-1 }

    // don't add unnecessary parens in the suggestion
    let ptr = &x as *const u8;
    let ptr_addr = ptr as usize;
// { dg-error "" "" { target *-*-* } .-1 }
    let ptr_addr_32bit = ptr as u32;
// { dg-error "" "" { target *-*-* } .-1 }
}


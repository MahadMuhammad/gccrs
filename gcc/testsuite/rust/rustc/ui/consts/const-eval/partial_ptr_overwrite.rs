// Test for the behavior described in <https://github.com/rust-lang/rust/issues/87184>.
#![feature(const_mut_refs)]

const PARTIAL_OVERWRITE: () = {
    let mut p = &42;
    unsafe {
        let ptr: *mut _ = &mut p;
        *(ptr as *mut u8) = 123; // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
    }
    let x = *p;
};

fn main() {}


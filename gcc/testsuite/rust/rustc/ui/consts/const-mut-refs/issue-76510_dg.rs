use std::mem::{transmute, ManuallyDrop};

const S: &'static mut str = &mut " hello ";
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

const fn trigger() -> [(); unsafe {
        let s = transmute::<(*const u8, usize), &ManuallyDrop<str>>((S.as_ptr(), 3));
        0
    }] {
    [(); 0]
}

fn main() {}


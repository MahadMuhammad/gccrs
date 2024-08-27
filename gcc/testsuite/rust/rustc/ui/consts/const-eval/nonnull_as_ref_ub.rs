use std::ptr::NonNull;

const NON_NULL: NonNull<u8> = unsafe { NonNull::dangling() };
const _: () = assert!(42 == *unsafe { NON_NULL.as_ref() }); // { dg-error ".E0080." "" { target *-*-* } }

fn main() {}


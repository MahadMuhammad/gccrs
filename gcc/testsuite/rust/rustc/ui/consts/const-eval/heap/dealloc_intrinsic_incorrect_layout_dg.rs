#![feature(core_intrinsics)]
#![feature(const_heap)]

use std::intrinsics;

const _X: () = unsafe {
    let ptr = intrinsics::const_allocate(4, 4);
    intrinsics::const_deallocate(ptr, 4, 2);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
};
const _Y: () = unsafe {
    let ptr = intrinsics::const_allocate(4, 4);
    intrinsics::const_deallocate(ptr, 2, 4);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
};

const _Z: () = unsafe {
    let ptr = intrinsics::const_allocate(4, 4);
    intrinsics::const_deallocate(ptr, 3, 4);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
};

const _W: () = unsafe {
    let ptr = intrinsics::const_allocate(4, 4);
    intrinsics::const_deallocate(ptr, 4, 3);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
};

fn main() {}


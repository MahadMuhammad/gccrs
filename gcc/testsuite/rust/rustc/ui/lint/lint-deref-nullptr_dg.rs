// test the deref_nullptr lint

#![deny(deref_nullptr)]

use std::ptr;

struct Struct {
    field: u8,
}

fn f() {
    unsafe {
        let a = 1;
        let ub = *(a as *const i32);
        let ub = *(0 as *const i32);
// { dg-error "" "" { target *-*-* } .-1 }
        let ub = *ptr::null::<i32>();
// { dg-error "" "" { target *-*-* } .-1 }
        let ub = *ptr::null_mut::<i32>();
// { dg-error "" "" { target *-*-* } .-1 }
        let ub = *(ptr::null::<i16>() as *const i32);
// { dg-error "" "" { target *-*-* } .-1 }
        let ub = *(ptr::null::<i16>() as *mut i32 as *mut usize as *const u8);
// { dg-error "" "" { target *-*-* } .-1 }
        let ub = &*ptr::null::<i32>();
// { dg-error "" "" { target *-*-* } .-1 }
        let ub = &*ptr::null_mut::<i32>();
// { dg-error "" "" { target *-*-* } .-1 }
        ptr::addr_of!(*ptr::null::<i32>());
// { dg-error "" "" { target *-*-* } .-1 }
        ptr::addr_of_mut!(*ptr::null_mut::<i32>());
// { dg-error "" "" { target *-*-* } .-1 }
        let offset = ptr::addr_of!((*ptr::null::<Struct>()).field);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}


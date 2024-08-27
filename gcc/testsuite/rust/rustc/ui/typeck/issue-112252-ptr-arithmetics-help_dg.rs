//@ run-rustfix

fn main() {
    let _ptr1: *const u32 = std::ptr::null();
    let _ptr2: *const u32 = std::ptr::null();
    let _a = _ptr1 + 5; // { dg-error ".E0369." "" { target *-*-* } }
    let _b = _ptr1 - 5; // { dg-error ".E0369." "" { target *-*-* } }
    let _c = _ptr2 - _ptr1; // { dg-error ".E0369." "" { target *-*-* } }
    let _d = _ptr1[5]; // { dg-error ".E0608." "" { target *-*-* } }
}


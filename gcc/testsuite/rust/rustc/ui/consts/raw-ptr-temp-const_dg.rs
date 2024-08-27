// A variant of raw-ptr-const that directly constructs a raw pointer.

const CONST_RAW: *const Vec<i32> = std::ptr::addr_of!(Vec::new());
// { dg-error ".E0745." "" { target *-*-* } .-1 }

fn main() {}


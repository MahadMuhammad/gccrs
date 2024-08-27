fn main() {}

// fine
const Z: i32 = unsafe { *(&1 as *const i32) };

// bad, will thus error in miri
const Z2: i32 = unsafe { *(42 as *const i32) }; // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
const Z3: i32 = unsafe { *(44 as *const i32) }; // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }


#[doc(inline)]
// { dg-error "" "" { target *-*-* } .-1 }
#[doc(no_inline)]
pub extern crate core;

// no warning
pub extern crate alloc;

fn main() {}


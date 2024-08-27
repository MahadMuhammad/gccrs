// This causes a query cycle due to using `Reveal::All`,
// in #119821 const eval was changed to always use `Reveal::All`
//
// See that PR for more details.
use std::mem::transmute;
fn foo() -> impl Sized {
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-warning ".E0391." "" { target *-*-* } .-2 }
    unsafe {
        transmute::<_, u8>(foo());
    }
    0u8
}

fn main() {}


#![allow(non_fmt_panics)]
#![crate_type = "lib"]

const MSG: &str = "hello";

const Z: () = std::panic!("cheese");
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const Z2: () = std::panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const Y: () = std::unreachable!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const X: () = std::unimplemented!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const W: () = std::panic!(MSG);
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const W2: () = std::panic!("{}", MSG);
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const Z_CORE: () = core::panic!("cheese");
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const Z2_CORE: () = core::panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const Y_CORE: () = core::unreachable!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const X_CORE: () = core::unimplemented!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const W_CORE: () = core::panic!(MSG);
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const W2_CORE: () = core::panic!("{}", MSG);
// { dg-error ".E0080." "" { target *-*-* } .-1 }


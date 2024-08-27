// { dg-additional-options "-frust-edition=2021" }
#![crate_type = "lib"]

const MSG: &str = "hello";

const A: () = std::panic!("blåhaj");
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const B: () = std::panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const C: () = std::unreachable!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const D: () = std::unimplemented!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const E: () = std::panic!("{}", MSG);
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const A_CORE: () = core::panic!("shark");
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const B_CORE: () = core::panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const C_CORE: () = core::unreachable!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const D_CORE: () = core::unimplemented!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const E_CORE: () = core::panic!("{}", MSG);
// { dg-error ".E0080." "" { target *-*-* } .-1 }


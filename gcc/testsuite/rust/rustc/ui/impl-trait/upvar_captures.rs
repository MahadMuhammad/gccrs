//! This test used to ICE: rust-lang/rust#123255
//! Because the errors on `C` were ignored when trying
//! to compute the MIR of the closure, which thus ended
//! up with broken upvars.
// { dg-additional-options "-frust-edition=2021" }
#![crate_type = "lib"]

pub fn a() {}

mod handlers {
    pub struct C(&()); // { dg-error ".E0106." "" { target *-*-* } }
    pub fn c() -> impl Fn() -> C {
        let a1 = ();
        || C((crate::a(), a1).into())
    }
}


//@ aux-crate:priv:priv_dep=priv_dep.rs
//@ aux-build:pub_dep.rs
//@ aux-crate:priv:pm=pm.rs
//@ compile-flags: -Zunstable-options

// Basic behavior check of exported_private_dependencies from either a public
// dependency or a private one.

#![deny(exported_private_dependencies)]

// This crate is a private dependency
// FIXME: This should trigger.
pub extern crate priv_dep;
// This crate is a public dependency
extern crate pub_dep;
// This crate is a private dependency
extern crate pm;

use priv_dep::{OtherTrait, OtherType};
use pub_dep::PubType;

// Type from private dependency used in private
// type - this is fine
struct PrivateType {
    field: OtherType,
}

pub struct PublicType {
    pub field: OtherType,
// { dg-error "" "" { target *-*-* } .-1 }
    priv_field: OtherType,    // Private field - this is fine
    pub other_field: PubType, // Type from public dependency - this is fine
}

impl PublicType {
    pub fn pub_fn_param(param: OtherType) {}
// { dg-error "" "" { target *-*-* } .-1 }

    pub fn pub_fn_return() -> OtherType { OtherType }
// { dg-error "" "" { target *-*-* } .-1 }

    fn priv_fn(param: OtherType) {}
}

pub trait MyPubTrait {
    type Foo: OtherTrait;
}
// { dg-error "" "" { target *-*-* } .-2 }

pub trait WithSuperTrait: OtherTrait {}
// { dg-error "" "" { target *-*-* } .-1 }

pub trait PubLocalTraitWithAssoc {
    type X;
}

pub struct PrivateAssoc;
impl PubLocalTraitWithAssoc for PrivateAssoc {
    type X = OtherType;
// { dg-error "" "" { target *-*-* } .-1 }
}

pub fn in_bounds<T: OtherTrait>(x: T) { unimplemented!() }
// { dg-error "" "" { target *-*-* } .-1 }

pub fn private_in_generic() -> std::num::Saturating<OtherType> { unimplemented!() }
// { dg-error "" "" { target *-*-* } .-1 }

pub static STATIC: OtherType = OtherType;
// { dg-error "" "" { target *-*-* } .-1 }

pub const CONST: OtherType = OtherType;
// { dg-error "" "" { target *-*-* } .-1 }

pub type Alias = OtherType;
// { dg-error "" "" { target *-*-* } .-1 }

pub struct PublicWithPrivateImpl;

// FIXME: This should trigger.
// See https://github.com/rust-lang/rust/issues/71043
impl OtherTrait for PublicWithPrivateImpl {}

pub trait PubTraitOnPrivate {}

// FIXME: This should trigger.
// See https://github.com/rust-lang/rust/issues/71043
impl PubTraitOnPrivate for OtherType {}

pub struct AllowedPrivType {
    #[allow(exported_private_dependencies)]
    pub allowed: OtherType,
}

// FIXME: This should trigger.
pub use priv_dep::m;
// FIXME: This should trigger.
pub use pm::fn_like;
// FIXME: This should trigger.
pub use pm::PmDerive;
// FIXME: This should trigger.
pub use pm::pm_attr;

// FIXME: This should trigger.
pub use priv_dep::E::V1;

fn main() {}


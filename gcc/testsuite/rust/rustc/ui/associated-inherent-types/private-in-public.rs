//@ check-pass

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]
#![crate_type = "lib"]

pub type PubAlias0 = PubTy::PrivAssocTy;
// { dg-warning "" "" { target *-*-* } .-1 }
pub type PubAlias1 = PrivTy::PubAssocTy;
// { dg-warning "" "" { target *-*-* } .-1 }
pub type PubAlias2 = PubTy::PubAssocTy<PrivTy>;
// { dg-warning "" "" { target *-*-* } .-1 }

pub struct PubTy;
impl PubTy {
    type PrivAssocTy = ();
    pub type PubAssocTy<T> = T;
}

struct PrivTy;
impl PrivTy {
    pub type PubAssocTy = ();
}


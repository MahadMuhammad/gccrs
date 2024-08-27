//@ revisions: stock gce

#![feature(associated_const_equality)]
#![cfg_attr(gce, feature(generic_const_exprs))]
// { dg-warning "" "" { target *-*-* } .-1 }

trait TraitWAssocConst {
    const A: usize;
}

fn foo<T: TraitWAssocConst<A = 1>>() {}

fn bar<T: TraitWAssocConst<A = 0>>() {
    foo::<T>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


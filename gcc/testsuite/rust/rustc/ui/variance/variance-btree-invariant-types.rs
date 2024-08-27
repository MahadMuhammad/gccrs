use std::collections::btree_map::{IterMut, OccupiedEntry, RangeMut, VacantEntry};

fn iter_cov_key<'a, 'new>(v: IterMut<'a, &'static (), ()>) -> IterMut<'a, &'new (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn iter_cov_val<'a, 'new>(v: IterMut<'a, (), &'static ()>) -> IterMut<'a, (), &'new ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn iter_contra_key<'a, 'new>(v: IterMut<'a, &'new (), ()>) -> IterMut<'a, &'static (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn iter_contra_val<'a, 'new>(v: IterMut<'a, (), &'new ()>) -> IterMut<'a, (), &'static ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}

fn range_cov_key<'a, 'new>(v: RangeMut<'a, &'static (), ()>) -> RangeMut<'a, &'new (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn range_cov_val<'a, 'new>(v: RangeMut<'a, (), &'static ()>) -> RangeMut<'a, (), &'new ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn range_contra_key<'a, 'new>(v: RangeMut<'a, &'new (), ()>) -> RangeMut<'a, &'static (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn range_contra_val<'a, 'new>(v: RangeMut<'a, (), &'new ()>) -> RangeMut<'a, (), &'static ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}

fn occ_cov_key<'a, 'new>(v: OccupiedEntry<'a, &'static (), ()>)
                         -> OccupiedEntry<'a, &'new (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn occ_cov_val<'a, 'new>(v: OccupiedEntry<'a, (), &'static ()>)
                         -> OccupiedEntry<'a, (), &'new ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn occ_contra_key<'a, 'new>(v: OccupiedEntry<'a, &'new (), ()>)
                            -> OccupiedEntry<'a, &'static (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn occ_contra_val<'a, 'new>(v: OccupiedEntry<'a, (), &'new ()>)
                            -> OccupiedEntry<'a, (), &'static ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}

fn vac_cov_key<'a, 'new>(v: VacantEntry<'a, &'static (), ()>)
                         -> VacantEntry<'a, &'new (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn vac_cov_val<'a, 'new>(v: VacantEntry<'a, (), &'static ()>)
                         -> VacantEntry<'a, (), &'new ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn vac_contra_key<'a, 'new>(v: VacantEntry<'a, &'new (), ()>)
                            -> VacantEntry<'a, &'static (), ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}
fn vac_contra_val<'a, 'new>(v: VacantEntry<'a, (), &'new ()>)
                            -> VacantEntry<'a, (), &'static ()> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}


fn main() { }


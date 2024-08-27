//@ run-rustfix

#![feature(lazy_type_alias)]
#![allow(incomplete_features)]
#![crate_type = "lib"]

// Check that we *reject* leading where-clauses on lazy type aliases.

pub type Leading0<T>
where // { dg-error "" "" { target *-*-* } }
    String: From<T>,
= T;

pub type Leading1<T, U>
where // { dg-error "" "" { target *-*-* } }
    String: From<T>,
= (T, U)
where
    U: Copy;

pub type EmptyLeading0 where = ();
// { dg-error "" "" { target *-*-* } .-1 }

pub type EmptyLeading1<T> where = T where T: Copy;
// { dg-error "" "" { target *-*-* } .-1 }


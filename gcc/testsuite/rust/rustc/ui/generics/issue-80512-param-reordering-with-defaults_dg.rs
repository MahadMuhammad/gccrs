#![crate_type = "lib"]

struct S<T = (), 'a>(&'a T);
// { dg-error "" "" { target *-*-* } .-1 }


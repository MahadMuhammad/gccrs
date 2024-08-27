#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: usize=3, T> {
// { dg-error "" "" { target *-*-* } .-1 }
  v: T
}


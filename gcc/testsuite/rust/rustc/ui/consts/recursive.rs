#![allow(unused)]

const fn f<T>(x: T) { // { dg-warning "" "" { target *-*-* } }
    f(x);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
}

const X: () = f(1);

fn main() {}


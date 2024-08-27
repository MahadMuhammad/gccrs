pub fn foo<Const N: u8>() {}
// { dg-error "" "" { target *-*-* } .-1 }

pub fn bar<Const>() {}
// OK

pub fn baz<Const N: u8, T>() {}
// { dg-error "" "" { target *-*-* } .-1 }

pub fn qux<T, Const N: u8>() {}
// { dg-error "" "" { target *-*-* } .-1 }

pub fn quux<T, Const N: u8, U>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


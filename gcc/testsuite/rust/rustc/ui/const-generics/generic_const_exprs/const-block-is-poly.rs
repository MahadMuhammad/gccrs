#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn foo<T>() {
    let _ = [0u8; const { std::mem::size_of::<T>() }];
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    foo::<i32>();
}


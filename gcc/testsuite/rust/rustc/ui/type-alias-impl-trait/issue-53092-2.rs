#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

type Bug<T, U> = impl Fn(T) -> U + Copy; // { dg-error ".E0391." "" { target *-*-* } }

const CONST_BUG: Bug<u8, ()> = unsafe { std::mem::transmute(|_: u8| ()) };
// { dg-error ".E0792." "" { target *-*-* } .-1 }
// { dg-error ".E0792." "" { target *-*-* } .-2 }
// { dg-error ".E0792." "" { target *-*-* } .-3 }

fn make_bug<T, U: From<T>>() -> Bug<T, U> {
    |x| x.into() // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {
    CONST_BUG(0);
}


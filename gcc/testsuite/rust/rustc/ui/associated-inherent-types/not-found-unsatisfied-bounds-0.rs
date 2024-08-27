// Regression test for issue #104251.

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Container<T: ?Sized>(T);

impl<T> Container<T> {
    type Yield = i32;
}

struct Duple<T, U>(T, U);

impl<T: Copy, U: Send> Duple<T, U> {
    type Combination = (T, U);
}

fn main() {
    let _: Container<[u8]>::Yield = 1; // { dg-error "" "" { target *-*-* } }
    let _: Duple<String, std::rc::Rc<str>>::Combination; // { dg-error "" "" { target *-*-* } }
}


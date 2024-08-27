// Regression test for issue #105305 and for
// https://github.com/rust-lang/rust/issues/107468#issuecomment-1409096700

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct S<T>(T);

impl<T, 'a> S<T> { // { dg-error "" "" { target *-*-* } }
    type P = T;
}

struct Subj<T>(T);

impl<T, S> Subj<(T, S)> {
    type Un = (T, S);
}

fn main() {
    let _: S<()>::P;

    let _: Subj<(i32, i32)>::Un = 0i32; // { dg-error ".E0308." "" { target *-*-* } }
}


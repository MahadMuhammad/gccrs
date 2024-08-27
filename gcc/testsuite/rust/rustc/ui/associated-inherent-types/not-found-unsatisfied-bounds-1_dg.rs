// fail-check

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

// Test if we use the correct `ParamEnv` when proving obligations.

fn parameterized<T>() {
    let _: Container<T>::Proj = String::new(); // { dg-error "" "" { target *-*-* } }
}

struct Container<T>(T);

impl<T: Clone> Container<T> {
    type Proj = String;
}

fn main() {}


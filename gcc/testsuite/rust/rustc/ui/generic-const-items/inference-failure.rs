#![feature(generic_const_items)]
#![allow(incomplete_features)]

const NONE<T>: Option<T> = None::<T>;
const IGNORE<T>: () = ();

fn none() {
    let _ = NONE; // { dg-error ".E0282." "" { target *-*-* } }
}

fn ignore() {
    let _ = IGNORE; // { dg-error ".E0282." "" { target *-*-* } }
}

fn main() {}


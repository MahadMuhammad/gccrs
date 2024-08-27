#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

struct Tagged<T: Tag, O: Options>;
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { help ".E0404." "" { target *-*-* } .-2 }
// { dg-error ".E0404." "" { target *-*-* } .-3 }
// { help ".E0404." "" { target *-*-* } .-4 }

#[derive(PartialEq, Eq, ConstParamTy)]
enum Tag {
    One,
    Two,
}

#[derive(PartialEq, Eq, ConstParamTy)]
struct Options {
    verbose: bool,
    safe: bool,
}

fn main() {}


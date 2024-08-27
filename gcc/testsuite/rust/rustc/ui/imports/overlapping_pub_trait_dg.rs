//@ aux-build:overlapping_pub_trait_source.rs

/*
 * This crate declares two public paths, `m::Tr` and `prelude::_`. Make sure we prefer the former.
 */
extern crate overlapping_pub_trait_source;
// { help "" "" { target *-*-* } .-1 }
// { suggestion "" "" { target *-*-* } .-2 }

fn main() {
    use overlapping_pub_trait_source::S;
    S.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}


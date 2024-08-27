//@ aux-build:unnamed_pub_trait_source.rs

/*
 * This crate declares an unnameable public path for our item. Make sure we don't suggest
 * importing it by name, and instead we suggest importing it by glob.
 */
extern crate unnamed_pub_trait_source;
// { help "" "" { target *-*-* } .-1 }
// { suggestion "" "" { target *-*-* } .-2 }

fn main() {
    use unnamed_pub_trait_source::S;
    S.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}


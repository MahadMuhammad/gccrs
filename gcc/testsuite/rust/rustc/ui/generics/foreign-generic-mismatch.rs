//@ aux-build: foreign-generic-mismatch.rs

extern crate foreign_generic_mismatch;

fn main() {
    foreign_generic_mismatch::const_arg::<()>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    foreign_generic_mismatch::lt_arg::<'static, 'static>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}


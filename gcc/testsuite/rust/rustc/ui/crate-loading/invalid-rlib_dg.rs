//@ compile-flags: --crate-type lib --extern foo={{src-base}}/crate-loading/auxiliary/libfoo.rlib
//@ normalize-stderr-test: "failed to mmap file '.*auxiliary/libfoo.rlib':.*" -> "failed to mmap file 'auxiliary/libfoo.rlib'"
// don't emit warn logging, it's basically the same as the errors and it's annoying to normalize
//@ rustc-env:RUSTC_LOG=error
// { dg-additional-options "-frust-edition=2018" }
#![no_std]
use ::foo; // { dg-error ".E0786." "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-error ".E0786." "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }


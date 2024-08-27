//@ aux-build:custom-quote.rs
//@ aux-build:span-from-proc-macro.rs
//@ compile-flags: -Z macro-backtrace

#[macro_use]
extern crate span_from_proc_macro;

#[error_from_attribute] // { dg-error "" "" { target *-*-* } }
struct ShouldBeRemoved;

#[derive(ErrorFromDerive)] // { dg-error "" "" { target *-*-* } }
struct Kept;

fn main() {
    error_from_bang!(); // { dg-error ".E0308." "" { target *-*-* } }
    other_error_from_bang!(); // { dg-error ".E0308." "" { target *-*-* } }
}


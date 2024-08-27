#![feature(coverage_attribute)]
// { dg-additional-options "-frust-edition= 2021" }

// Tests the error messages produced (or not produced) by various unusual
// uses of the `#[coverage(..)]` attribute.

#[coverage(off)] // { dg-error "" "" { target *-*-* } }
#[coverage(off)]
fn multiple_consistent() {}

#[coverage(off)] // { dg-error "" "" { target *-*-* } }
#[coverage(on)]
fn multiple_inconsistent() {}

#[coverage] // { dg-error "" "" { target *-*-* } }
fn bare_word() {}

#[coverage = true] // { dg-error "" "" { target *-*-* } }
fn key_value() {}

#[coverage()] // { dg-error "" "" { target *-*-* } }
fn list_empty() {}

#[coverage(off, off)] // { dg-error "" "" { target *-*-* } }
fn list_consistent() {}

#[coverage(off, on)] // { dg-error "" "" { target *-*-* } }
fn list_inconsistent() {}

#[coverage(bogus)] // { dg-error "" "" { target *-*-* } }
fn bogus_word() {}

#[coverage(bogus, off)] // { dg-error "" "" { target *-*-* } }
fn bogus_word_before() {}

#[coverage(off, bogus)] // { dg-error "" "" { target *-*-* } }
fn bogus_word_after() {}

#[coverage(off,)] // (OK!)
fn comma_after() {}

#[coverage(,off)] // { dg-error "" "" { target *-*-* } }
fn comma_before() {}

fn main() {}


#![deny(unused_attributes)]
// Unused attributes on macro_rules requires special handling since the
// macro_rules definition does not survive towards HIR.

// A sample of various built-in attributes.
#[macro_export]
#[macro_use] // { dg-error "" "" { target *-*-* } }
#[path="foo"] // { dg-error "" "" { target *-*-* } }
#[recursion_limit="1"] // { dg-error "" "" { target *-*-* } }
macro_rules! foo {
    () => {};
}

// The following should not warn about unused attributes.
#[allow(unused)]
macro_rules! foo2 {
    () => {};
}

#[cfg(FALSE)]
macro_rules! foo {
    () => {};
}

/// Some docs
#[deprecated]
#[doc = "more docs"]
#[macro_export]
macro_rules! bar {
    () => {};
}

fn main() {}


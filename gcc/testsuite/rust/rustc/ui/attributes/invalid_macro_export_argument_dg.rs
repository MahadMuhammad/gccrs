//@ revisions: deny allow
//@[allow] check-pass

#![cfg_attr(deny, deny(invalid_macro_export_arguments))]
#![cfg_attr(allow, allow(invalid_macro_export_arguments))]

#[macro_export(hello, world)]
// { dg-error "" "" { target *-*-* } .-1 }
macro_rules! a {
    () => ()
}

#[macro_export(not_local_inner_macros)]
// { dg-error "" "" { target *-*-* } .-1 }
macro_rules! b {
    () => ()
}

#[macro_export]
macro_rules! c {
    () => ()
}
#[macro_export(local_inner_macros)]
macro_rules! d {
    () => ()
}

#[macro_export()]
macro_rules! e {
    () => ()
}

fn main() {}


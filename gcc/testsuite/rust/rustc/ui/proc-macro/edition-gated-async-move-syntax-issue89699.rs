//@ aux-build:edition-gated-async-move-syntax.rs
// { dg-additional-options "-frust-edition= 2015" }

// Non-regression test for issue #89699, where a proc-macro emitting syntax only available in
//@ edition 2018 and up (`async move`) is used on edition 2015

extern crate edition_gated_async_move_syntax;

#[edition_gated_async_move_syntax::foo]
fn foo() {} // { dg-error "" "" { target *-*-* } }


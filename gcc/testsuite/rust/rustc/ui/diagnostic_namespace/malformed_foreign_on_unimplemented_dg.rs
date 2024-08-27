// { dg-additional-options "-frust-edition=2021" }
//@ aux-build:bad_on_unimplemented.rs

// Do not ICE when encountering a malformed `#[diagnostic::on_unimplemented]` annotation in a
// dependency when incorrectly used (#124651).

extern crate bad_on_unimplemented;

use bad_on_unimplemented::*;

fn missing_attr<T: MissingAttr>(_: T) {}
fn duplicate_attr<T: DuplicateAttr>(_: T) {}
fn not_meta_list<T: NotMetaList>(_: T) {}
fn empty<T: Empty>(_: T) {}
fn wrong_delim<T: WrongDelim>(_: T) {}
fn bad_formatter<T: BadFormatter<()>>(_: T) {}
fn no_implicit_args<T: NoImplicitArgs>(_: T) {}
fn missing_arg<T: MissingArg>(_: T) {}
fn bad_arg<T: BadArg>(_: T) {}

fn main() {
    missing_attr(()); // { dg-error ".E0277." "" { target *-*-* } }
    duplicate_attr(()); // { dg-error ".E0277." "" { target *-*-* } }
    not_meta_list(()); // { dg-error ".E0277." "" { target *-*-* } }
    empty(()); // { dg-error ".E0277." "" { target *-*-* } }
    wrong_delim(()); // { dg-error ".E0277." "" { target *-*-* } }
    bad_formatter(()); // { dg-error ".E0277." "" { target *-*-* } }
    no_implicit_args(()); // { dg-error ".E0277." "" { target *-*-* } }
    missing_arg(()); // { dg-error ".E0277." "" { target *-*-* } }
    bad_arg(()); // { dg-error ".E0277." "" { target *-*-* } }
}


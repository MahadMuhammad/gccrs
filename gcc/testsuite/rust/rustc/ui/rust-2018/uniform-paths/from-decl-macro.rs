//@ build-pass (FIXME(62277): could be check-pass?)
// { dg-additional-options "-frust-edition=2018" }

#![feature(decl_macro)]

macro check() {
    ::std::vec::Vec::<u8>::new()
}

fn main() {
    check!();
}


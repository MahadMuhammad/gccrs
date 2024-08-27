//@ compile-flags: -Z unstable-options

#![feature(rustdoc_internals)]

#![crate_type = "lib"]

#![deny(rustc::existing_doc_keyword)]

#[doc(keyword = "tadam")] // { dg-error "" "" { target *-*-* } }
mod tadam {}


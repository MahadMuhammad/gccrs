// gate-test-const_async_blocks

// { dg-additional-options "-frust-edition=2018" }
//@ revisions: with_feature without_feature

#![feature(rustc_attrs)]
#![cfg_attr(with_feature, feature(const_async_blocks))]

use std::future::Future;

// From <https://github.com/rust-lang/rust/issues/77361>
const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
// { dg-error "" "" { target *-*-* } .-1 }

static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
// { dg-error "" "" { target *-*-* } .-1 }

#[rustc_error]
fn main() {} // { dg-error "" "" { target *-*-* } }


//@ aux-build:send_sync.rs

#![feature(trait_alias)]

extern crate send_sync;

use std::rc::Rc;
use send_sync::SendSync;

fn use_alias<T: SendSync>() {}

fn main() {
    use_alias::<u32>();
    use_alias::<Rc<u32>>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}


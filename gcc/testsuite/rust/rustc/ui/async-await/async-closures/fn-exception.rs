// { dg-additional-options "-frust-edition= 2021" }

#![feature(async_closure)]

use std::pin::Pin;
use std::future::Future;

unsafe extern "Rust" {
    pub unsafe fn unsafety() -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}

unsafe extern "C" {
    pub safe fn abi() -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}

fn test(f: impl async Fn()) {}

fn main() {
    test(unsafety); // { dg-error ".E0277." "" { target *-*-* } }
    test(abi); // { dg-error ".E0277." "" { target *-*-* } }
}


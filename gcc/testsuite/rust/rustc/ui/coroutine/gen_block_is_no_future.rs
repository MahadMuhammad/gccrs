//@compile-flags: --edition 2024 -Zunstable-options
#![feature(gen_blocks)]

fn foo() -> impl std::future::Future { // { dg-error ".E0277." "" { target *-*-* } }
    gen { yield 42 }
}

fn main() {}


#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::Coroutine;
use std::pin::Pin;

fn main() {
    let s = String::from("foo");
    let mut gen = #[coroutine]
    move || {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        yield s[..];
    };
    Pin::new(&mut gen).resume(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


#![feature(coroutine_trait)]
#![feature(coroutines, stmt_expr_attributes)]
#![deny(unused_braces, unused_parens)]

use std::ops::Coroutine;
use std::pin::Pin;

fn main() {
    let mut x = #[coroutine] |_| {
        while let Some(_) = (yield) {}
        while let Some(_) = {yield} {}

        // Only warn these cases
        while let Some(_) = ({yield}) {} // { dg-error "" "" { target *-*-* } }
        while let Some(_) = ((yield)) {} // { dg-error "" "" { target *-*-* } }
        {{yield}}; // { dg-error "" "" { target *-*-* } }
        {( yield )}; // { dg-error "" "" { target *-*-* } }
        while let Some(_) = {(yield)} {} // { dg-error "" "" { target *-*-* } }
        while let Some(_) = {{yield}} {} // { dg-error "" "" { target *-*-* } }

        // FIXME: It'd be great if we could also warn them.
        ((yield));
        ({ yield });
    };
    let _ = Pin::new(&mut x).resume(Some(5));
}


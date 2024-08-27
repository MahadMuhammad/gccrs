// Expression macros can't expand to a let match guard.

#![feature(if_let_guard)]
#![feature(let_chains)]

macro_rules! m {
    ($e:expr) => { let Some(x) = $e }
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    match () {
        () if m!(Some(5)) => {}
        _ => {}
    }
}


// gate-test-let_chains

// Here we test feature gating for ´let_chains`.
// See `disallowed-positions.rs` for the grammar
// defining the language for gated allowed positions.

#![allow(irrefutable_let_patterns)]

use std::ops::Range;

fn _if() {
    if let 0 = 1 {} // Stable!

    if true && let 0 = 1 {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    if let 0 = 1 && true {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    if let Range { start: _, end: _ } = (true..true) && false {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    if let 1 = 1 && let true = { true } && false {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    }
}

fn _while() {
    while let 0 = 1 {} // Stable!

    while true && let 0 = 1 {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    while let 0 = 1 && true {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    while let Range { start: _, end: _ } = (true..true) && false {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

fn _macros() {
    macro_rules! noop_expr { ($e:expr) => {}; }

    noop_expr!((let 0 = 1));
// { dg-error "" "" { target *-*-* } .-1 }

    macro_rules! use_expr {
        ($e:expr) => {
            if $e {}
            while $e {}
        }
    }
    #[cfg(FALSE)] (let 0 = 1);
// { dg-error "" "" { target *-*-* } .-1 }
    use_expr!(let 0 = 1);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


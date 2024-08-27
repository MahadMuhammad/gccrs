//@ compile-flags: -Zdeduplicate-diagnostics=yes

// Macros were previously expanded in `Expr` nonterminal tokens, now they are not.

macro_rules! pass_nonterminal {
    ($n:expr) => {
        #[repr(align($n))]
// { dg-error "" "" { target *-*-* } .-1 }
        struct S;
    };
}

macro_rules! n {
    () => { 32 };
}

pass_nonterminal!(n!());
// { dg-error ".E0693." "" { target *-*-* } .-1 }

fn main() {}


#![feature(macro_metavar_expr)]

macro_rules! foo {
    ( $( $($t:ident),* );* ) => { ${count($t,)} }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn test() {
    foo!(a, a; b, b);
}

fn main() {
}


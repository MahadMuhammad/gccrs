#![feature(macro_metavar_expr)]

// `curly` = Right hand side curly brackets
// `no_rhs_dollar` = No dollar sign at the right hand side meta variable "function"
// `round` = Left hand side round brackets

macro_rules! curly__no_rhs_dollar__round {
    ( $( $i:ident ),* ) => { ${ count($i) } };
}

macro_rules! curly__no_rhs_dollar__no_round {
    ( $i:ident ) => { ${ count($i) } };
// { dg-error "" "" { target *-*-* } .-1 }
}

macro_rules! curly__rhs_dollar__no_round {
    ( $i:ident ) => { ${ count($i) } };
// { dg-error "" "" { target *-*-* } .-1 }
}

#[rustfmt::skip] // autoformatters can break a few of the error traces
macro_rules! no_curly__no_rhs_dollar__round {
    ( $( $i:ident ),* ) => { count(i) };
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
}

#[rustfmt::skip] // autoformatters can break a few of the error traces
macro_rules! no_curly__no_rhs_dollar__no_round {
    ( $i:ident ) => { count(i) };
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
}

#[rustfmt::skip] // autoformatters can break a few of the error traces
macro_rules! no_curly__rhs_dollar__round {
    ( $( $i:ident ),* ) => { count($i) };
// { dg-error "" "" { target *-*-* } .-1 }
}

#[rustfmt::skip] // autoformatters can break a few of the error traces
macro_rules! no_curly__rhs_dollar__no_round {
    ( $i:ident ) => { count($i) };
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

// Other scenarios

macro_rules! dollar_dollar_in_the_lhs {
    ( $$ $a:ident ) => {
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

macro_rules! extra_garbage_after_metavar {
    ( $( $i:ident ),* ) => {
        ${count() a b c}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        ${count($i a b c)}
// { dg-error "" "" { target *-*-* } .-1 }
        ${count($i, 1 a b c)}
// { dg-error "" "" { target *-*-* } .-1 }
        ${count($i) a b c}
// { dg-error "" "" { target *-*-* } .-1 }

        ${ignore($i) a b c}
// { dg-error "" "" { target *-*-* } .-1 }
        ${ignore($i a b c)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${index() a b c}
// { dg-error "" "" { target *-*-* } .-1 }
        ${index(1 a b c)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${index() a b c}
// { dg-error "" "" { target *-*-* } .-1 }
        ${index(1 a b c)}
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

const IDX: usize = 1;
macro_rules! metavar_depth_is_not_literal {
    ( $( $i:ident ),* ) => { ${ index(IDX) } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

macro_rules! metavar_in_the_lhs {
    ( ${ len() } ) => {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    };
}

macro_rules! metavar_token_without_ident {
    ( $( $i:ident ),* ) => { ${ ignore() } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

macro_rules! metavar_with_literal_suffix {
    ( $( $i:ident ),* ) => { ${ index(1u32) } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

macro_rules! metavar_without_parens {
    ( $( $i:ident ),* ) => { ${ count{i} } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

#[rustfmt::skip]
macro_rules! open_brackets_without_tokens {
    ( $( $i:ident ),* ) => { ${ {} } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

macro_rules! unknown_count_ident {
    ( $( $i:ident )* ) => {
        ${count(foo)}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    };
}

macro_rules! unknown_ignore_ident {
    ( $( $i:ident )* ) => {
        ${ignore(bar)}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    };
}

macro_rules! unknown_metavar {
    ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {
    curly__no_rhs_dollar__round!(a, b, c);
    curly__no_rhs_dollar__no_round!(a);
    curly__rhs_dollar__no_round!(a);
    no_curly__no_rhs_dollar__round!(a, b, c);
    no_curly__no_rhs_dollar__no_round!(a);
    no_curly__rhs_dollar__round!(a, b, c);
    no_curly__rhs_dollar__no_round!(a);
// { dg-error ".E0425." "" { target *-*-* } .-1 }

    extra_garbage_after_metavar!(a);
    metavar_depth_is_not_literal!(a);
    metavar_token_without_ident!(a);
    metavar_with_literal_suffix!(a);
    metavar_without_parens!(a);
    open_brackets_without_tokens!(a);
    unknown_count_ident!(a);
    unknown_ignore_ident!(a);
    unknown_metavar!(a);
}


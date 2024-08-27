// Here we test that `lowering` behaves correctly wrt. `let $pats = $expr` expressions.
//
// We want to make sure that `let` is banned in situations other than:
//
// expr =
//   | ...
//   | "if" expr_with_let block {"else" block}?
//   | {label ":"}? while" expr_with_let block
//   ;
//
// expr_with_let =
//   | "let" top_pats "=" expr
//   | expr_with_let "&&" expr_with_let
//   | "(" expr_with_let ")"
//   | expr
//   ;
//
// To that end, we check some positions which is not part of the language above.

#![feature(let_chains)] // Avoid inflating `.stderr` with overzealous gates in this test.

#![allow(irrefutable_let_patterns)]

use std::ops::Range;

fn main() {}

fn _if() {
    if (let 0 = 1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if (((let 0 = 1))) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if (let 0 = 1) && true {}
// { dg-error "" "" { target *-*-* } .-1 }

    if true && (let 0 = 1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if (let 0 = 1) && (let 0 = 1) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

fn _while() {
    while (let 0 = 1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    while (((let 0 = 1))) {}
// { dg-error "" "" { target *-*-* } .-1 }

    while (let 0 = 1) && true {}
// { dg-error "" "" { target *-*-* } .-1 }

    while true && (let 0 = 1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    while (let 0 = 1) && (let 0 = 1) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

fn _macros() {
    macro_rules! use_expr {
        ($e:expr) => {
            if $e {}
            while $e {}
        }
    }
    use_expr!((let 0 = 1 && 0 == 0));
// { dg-error "" "" { target *-*-* } .-1 }
    use_expr!((let 0 = 1));
// { dg-error "" "" { target *-*-* } .-1 }
}

fn nested_within_if_expr() {
    if &let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }

    if !let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
    if *let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
    if -let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }

    fn _check_try_binds_tighter() -> Result<(), ()> {
        if let 0 = 0? {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        Ok(())
    }
    if (let 0 = 0)? {}
// { dg-error "" "" { target *-*-* } .-1 }

    if true || let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
    if (true || let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
    if true && (true || let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
    if true || (true && let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }

    let mut x = true;
    if x = let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }

    if true..(let 0 = 0) {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    if ..(let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
    if (let 0 = 0).. {}
// { dg-error "" "" { target *-*-* } .-1 }

    // Binds as `(let ... = true)..true &&/|| false`.
    if let Range { start: _, end: _ } = true..true && false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    if let Range { start: _, end: _ } = true..true || false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    // Binds as `(let Range { start: F, end } = F)..(|| true)`.
    const F: fn() -> bool = || true;
    if let Range { start: F, end } = F..|| true {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    // Binds as `(let Range { start: true, end } = t)..(&&false)`.
    let t = &&true;
    if let Range { start: true, end } = t..&&false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    if let true = let true = true {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn nested_within_while_expr() {
    while &let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }

    while !let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
    while *let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
    while -let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }

    fn _check_try_binds_tighter() -> Result<(), ()> {
        while let 0 = 0? {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        Ok(())
    }
    while (let 0 = 0)? {}
// { dg-error "" "" { target *-*-* } .-1 }

    while true || let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
    while (true || let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
    while true && (true || let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
    while true || (true && let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }

    let mut x = true;
    while x = let 0 = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }

    while true..(let 0 = 0) {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    while ..(let 0 = 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
    while (let 0 = 0).. {}
// { dg-error "" "" { target *-*-* } .-1 }

    // Binds as `(let ... = true)..true &&/|| false`.
    while let Range { start: _, end: _ } = true..true && false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    while let Range { start: _, end: _ } = true..true || false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    // Binds as `(let Range { start: F, end } = F)..(|| true)`.
    const F: fn() -> bool = || true;
    while let Range { start: F, end } = F..|| true {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    // Binds as `(let Range { start: true, end } = t)..(&&false)`.
    let t = &&true;
    while let Range { start: true, end } = t..&&false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    while let true = let true = true {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn not_error_because_clarified_intent() {
    if let Range { start: _, end: _ } = (true..true || false) { }

    if let Range { start: _, end: _ } = (true..true && false) { }

    while let Range { start: _, end: _ } = (true..true || false) { }

    while let Range { start: _, end: _ } = (true..true && false) { }
}

fn outside_if_and_while_expr() {
    &let 0 = 0;
// { dg-error "" "" { target *-*-* } .-1 }

    !let 0 = 0;
// { dg-error "" "" { target *-*-* } .-1 }
    *let 0 = 0;
// { dg-error "" "" { target *-*-* } .-1 }
    -let 0 = 0;
// { dg-error "" "" { target *-*-* } .-1 }

    fn _check_try_binds_tighter() -> Result<(), ()> {
        let 0 = 0?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        Ok(())
    }
    (let 0 = 0)?;
// { dg-error "" "" { target *-*-* } .-1 }

    true || let 0 = 0;
// { dg-error "" "" { target *-*-* } .-1 }
    (true || let 0 = 0);
// { dg-error "" "" { target *-*-* } .-1 }
    true && (true || let 0 = 0);
// { dg-error "" "" { target *-*-* } .-1 }

    let mut x = true;
    x = let 0 = 0;
// { dg-error "" "" { target *-*-* } .-1 }

    true..(let 0 = 0);
// { dg-error "" "" { target *-*-* } .-1 }
    ..(let 0 = 0);
// { dg-error "" "" { target *-*-* } .-1 }
    (let 0 = 0)..;
// { dg-error "" "" { target *-*-* } .-1 }

    (let Range { start: _, end: _ } = true..true || false);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    (let true = let true = true);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    {
        #[cfg(FALSE)]
        let x = true && let y = 1;
// { dg-error "" "" { target *-*-* } .-1 }
    }

    #[cfg(FALSE)]
    {
        [1, 2, 3][let _ = ()]
// { dg-error "" "" { target *-*-* } .-1 }
    }

    // Check function tail position.
    &let 0 = 0
// { dg-error "" "" { target *-*-* } .-1 }
}

// Let's make sure that `let` inside const generic arguments are considered.
fn inside_const_generic_arguments() {
    struct A<const B: bool>;
    impl<const B: bool> A<{B}> { const O: u32 = 5; }

    if let A::<{
        true && let 1 = 1
// { dg-error "" "" { target *-*-* } .-1 }
    }>::O = 5 {}

    while let A::<{
        true && let 1 = 1
// { dg-error "" "" { target *-*-* } .-1 }
    }>::O = 5 {}

    if A::<{
        true && let 1 = 1
// { dg-error "" "" { target *-*-* } .-1 }
    }>::O == 5 {}

    // In the cases above we have `ExprKind::Block` to help us out.
    // Below however, we would not have a block and so an implementation might go
    // from visiting expressions to types without banning `let` expressions down the tree.
    // This tests ensures that we are not caught by surprise should the parser
    // admit non-IDENT expressions in const generic arguments.

    if A::<
        true && let 1 = 1
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    >::O == 5 {}
}

fn with_parenthesis() {
    let opt = Some(Some(1i32));

    if (let Some(a) = opt && true) {
// { dg-error "" "" { target *-*-* } .-1 }
    }

    if (let Some(a) = opt) && true {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if (let Some(a) = opt) && (let Some(b) = a) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
    if let Some(a) = opt && (true && true) {
    }

    if (let Some(a) = opt && (let Some(b) = a)) && b == 1 {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
    if (let Some(a) = opt && (let Some(b) = a)) && true {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
    if (let Some(a) = opt && (true)) && true {
// { dg-error "" "" { target *-*-* } .-1 }
    }

    if (true && (true)) && let Some(a) = opt {
    }
    if (true) && let Some(a) = opt {
    }
    if true && let Some(a) = opt {
    }

    let fun = || true;
    if let true = (true && fun()) && (true) {
    }

    #[cfg(FALSE)]
    let x = (true && let y = 1);
// { dg-error "" "" { target *-*-* } .-1 }

    #[cfg(FALSE)]
    {
        ([1, 2, 3][let _ = ()])
// { dg-error "" "" { target *-*-* } .-1 }
    }
}


#![deny(unused_doc_comments)]

fn doc_comment_on_match_arms(num: u8) -> bool {
    match num {
        3 => true,
        /// useless doc comment
// { dg-error "" "" { target *-*-* } .-1 }
        _ => false,
    }
}

fn doc_comment_between_if_else(num: u8) -> bool {
    if num == 3 {
        true // { dg-error ".E0308." "" { target *-*-* } }
    }
    /// useless doc comment
    else { // { dg-error "" "" { target *-*-* } }
        false
    }
}

fn doc_comment_on_expr(num: u8) -> bool {
    /// useless doc comment
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    num == 3
}

fn doc_comment_on_expr_field() -> bool {
    struct S { foo: i32 }

    let x = S {
        /// useless doc comment
// { dg-error "" "" { target *-*-* } .-1 }
        foo: 3
    };

    true
}

fn doc_comment_on_pat_field() -> bool {
    struct S { foo: i32 }

    let S {
        /// useless doc comment
// { dg-error "" "" { target *-*-* } .-1 }
        foo
    } = S {
        foo: 3
    };

    true
}

fn doc_comment_on_generic<#[doc = "x"] T>(val: T) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn doc_comment_on_block() {
    /// unused doc comment
// { dg-error "" "" { target *-*-* } .-1 }
    {
        let x = 12;
    }
}

/// unused doc comment
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" {
    fn foo();
}

fn main() {}


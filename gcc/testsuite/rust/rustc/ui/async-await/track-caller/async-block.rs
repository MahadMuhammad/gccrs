// { dg-additional-options "-frust-edition=2021" }
//@ revisions: afn nofeat

#![feature(stmt_expr_attributes)]
#![cfg_attr(afn, feature(async_fn_track_caller))]

fn main() {
    let _ = #[track_caller] async {
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

#[track_caller]
async fn foo() {
    let _ = #[track_caller] async {
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

#[track_caller]
async fn foo2() {
    let _ = async {
        let _ = #[track_caller] async {
// { dg-error "" "" { target *-*-* } .-1 }
        };
    };
}


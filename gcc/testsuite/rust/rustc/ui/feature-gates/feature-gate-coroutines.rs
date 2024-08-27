//@ revisions: e2024 none
//@[e2024] compile-flags: --edition 2024 -Zunstable-options

fn main() {
    yield true; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

    let _ = || yield true; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

#[cfg(FALSE)]
fn foo() {
    // Ok in 2024 edition
    yield; // { dg-error "" "" { target *-*-* } }
    yield 0; // { dg-error "" "" { target *-*-* } }
}


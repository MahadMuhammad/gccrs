//@ revisions: e2024 none
//@[e2024] compile-flags: --edition 2024 -Zunstable-options

fn test_gen() {
    gen {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn test_async_gen() {
    async gen {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

#[cfg(FALSE)]
fn foo() {
    gen {};
// { dg-error "" "" { target *-*-* } .-1 }

    async gen {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}


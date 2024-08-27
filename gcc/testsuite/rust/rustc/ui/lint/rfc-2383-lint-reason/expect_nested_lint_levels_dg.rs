// ignore-tidy-linelength

#![warn(unused_mut)]

#[expect(
    unused_mut,
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    reason = "this `expect` is overridden by a `allow` attribute before the `unused_mut` lint is triggered"
)]
mod foo {
    fn bar() {
        #[allow(
            unused_mut,
            reason = "this overrides the previous `expect` lint level and allows the `unused_mut` lint here"
        )]
        let mut v = 0;
    }
}

#[expect(
    unused_mut,
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    reason = "this `expect` is overridden by a `warn` attribute before the `unused_mut` lint is triggered"
)]
mod oof {
    #[warn(
        unused_mut,
// { dg-note "" "" { target *-*-* } .-1 }
        reason = "this overrides the previous `expect` lint level and warns about the `unused_mut` lint here"
    )]
    fn bar() {
        let mut v = 0;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    }
}

#[expect(unused_variables)]
// { dg-warning "" "" { target *-*-* } .-1 }
#[forbid(unused_variables)]
// { dg-note "" "" { target *-*-* } .-1 }
fn check_expect_then_forbid() {
    let this_is_my_function = 3;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {}


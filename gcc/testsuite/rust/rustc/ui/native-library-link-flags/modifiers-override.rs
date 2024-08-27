//@ compile-flags:-ldylib:+as-needed=foo -lstatic=bar -Zunstable-options

#[link(name = "foo")]
#[link(
    name = "bar",
    kind = "static",
    modifiers = "+whole-archive,-whole-archive",
// { dg-error "" "" { target *-*-* } .-1 }
    modifiers = "+bundle"
// { dg-error "" "" { target *-*-* } .-1 }
)]
extern "C" {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}


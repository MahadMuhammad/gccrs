//@ aux-build:missing-assoc-fn-applicable-suggestions.rs

extern crate missing_assoc_fn_applicable_suggestions;
use missing_assoc_fn_applicable_suggestions::TraitA;

struct S;
impl TraitA<()> for S {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
}
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }

fn main() {}


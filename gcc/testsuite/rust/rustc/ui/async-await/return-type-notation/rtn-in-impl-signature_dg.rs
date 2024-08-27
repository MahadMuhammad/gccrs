#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

// Shouldn't ICE when we have a (bad) RTN in an impl header

trait Super1<'a> {
    fn bar<'b>() -> bool;
}

impl Super1<'_, bar(..): Send> for () {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }

fn main() {}


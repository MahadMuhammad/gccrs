//@ check-fail
//@ dont-check-compiler-stdout
//@ dont-check-compiler-stderr

fn�a<e>(){fn�p(){e}} // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }

fn main(){}


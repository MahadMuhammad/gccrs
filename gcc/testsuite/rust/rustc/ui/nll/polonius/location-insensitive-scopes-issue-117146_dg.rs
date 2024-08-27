// This is a non-regression test for issue #117146, where NLL and `-Zpolonius=next` computed
// different loan scopes when a region flowed into an SCC whose representative was an existential
// region.

//@ revisions: nll polonius
//@ [polonius] compile-flags: -Zpolonius=next

fn main() {
    let a = ();
    let b = |_| &a;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    bad(&b);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn bad<F: Fn(&()) -> &()>(_: F) {}


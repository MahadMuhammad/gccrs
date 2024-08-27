//@ revisions: base polonius
//@ ignore-compare-mode-polonius
//@ [polonius] compile-flags: -Z polonius

trait TT {}

impl dyn TT {
    fn func(&self) {}
}

fn main() {
    let f = |x: &dyn TT| x.func();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}


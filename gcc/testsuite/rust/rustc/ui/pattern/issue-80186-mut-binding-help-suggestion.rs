// Regression test for correct pretty-printing of an AST representing `&(mut x)` in help
// suggestion diagnostic.

fn main() {
    let mut &x = &0;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }
}


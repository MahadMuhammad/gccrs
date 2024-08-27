// Regression test for issue #91421.

fn main() {
    let value = if true && {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        3
    } else { 4 };
}


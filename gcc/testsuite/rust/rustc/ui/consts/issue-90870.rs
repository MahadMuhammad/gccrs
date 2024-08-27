// Regression test for issue #90870.

#![allow(dead_code)]

const fn f(a: &u8, b: &u8) -> bool {
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    a == b
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { help ".E0015." "" { target *-*-* } .-2 }
}

const fn g(a: &&&&i64, b: &&&&i64) -> bool {
    a == b
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { help ".E0015." "" { target *-*-* } .-2 }
}

const fn h(mut a: &[u8], mut b: &[u8]) -> bool {
    while let ([l, at @ ..], [r, bt @ ..]) = (a, b) {
        if l == r {
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { help ".E0015." "" { target *-*-* } .-2 }
            a = at;
            b = bt;
        } else {
            return false;
        }
    }

    a.is_empty() && b.is_empty()
}

fn main() {}


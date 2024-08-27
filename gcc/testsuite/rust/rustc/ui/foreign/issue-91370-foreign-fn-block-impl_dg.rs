// Regression test for issue #91370.

extern {
// { dg-error "" "" { target *-*-* } .-1 }
    fn f() {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        impl Copy for u8 {}
    }
}

fn main() {}


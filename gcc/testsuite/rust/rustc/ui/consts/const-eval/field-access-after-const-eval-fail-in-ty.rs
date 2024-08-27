// Regression test for issue #120615.

fn main() {
    [(); loop {}].field; // { dg-error "" "" { target *-*-* } }
}


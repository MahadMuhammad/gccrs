fn main() {
    { unsafe 92 } // { dg-error "" "" { target *-*-* } }
}

fn foo() {
    { mod 92 } // { dg-error "" "" { target *-*-* } }
}

fn bar() {
    { trait 92 } // { dg-error "" "" { target *-*-* } }
}


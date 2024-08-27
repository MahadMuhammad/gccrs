#![deny(redundant_semicolons)]

fn main() {
    fn inner() {}; // { dg-error "" "" { target *-*-* } }
    struct Bar {}; // { dg-error "" "" { target *-*-* } }
}


#![feature(builtin_syntax)]

fn main() {
    builtin # foobar(); // { dg-error "" "" { target *-*-* } }
}

fn not_identifier() {
    builtin # {}(); // { dg-error "" "" { target *-*-* } }
}


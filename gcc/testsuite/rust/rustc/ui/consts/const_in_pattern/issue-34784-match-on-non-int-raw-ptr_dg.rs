#![allow(dead_code)]

const C: *const u8 = &0;
// Make sure we also find pointers nested in other types.
const C_INNER: (*const u8, u8) = (C, 0);

fn foo(x: *const u8) {
    match x {
        C => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

fn foo2(x: *const u8) {
    match (x, 1) {
        C_INNER => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

const D: *const [u8; 4] = b"abcd";

const STR: *const str = "abcd";

fn main() {
    match D {
        D => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }

    match STR {
        STR => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}


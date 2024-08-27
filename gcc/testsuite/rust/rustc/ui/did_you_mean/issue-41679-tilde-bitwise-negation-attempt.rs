//@ run-rustfix

fn main() {
    let _x = ~1; // { dg-error "" "" { target *-*-* } }
    let _y = not 1; // { dg-error "" "" { target *-*-* } }
    let _z = not false; // { dg-error "" "" { target *-*-* } }
    let _a = not true; // { dg-error "" "" { target *-*-* } }
    let v = 1 + 2;
    let _v = not v; // { dg-error "" "" { target *-*-* } }
}


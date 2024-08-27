//@ run-rustfix
#[allow(unused_parens)]
fn main() {
    let _ = +1; // { dg-error "" "" { target *-*-* } }
    let _ = (1.0 + +2.0) * +3.0; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
    let _ = [+3, 4+6]; // { dg-error "" "" { target *-*-* } }
}


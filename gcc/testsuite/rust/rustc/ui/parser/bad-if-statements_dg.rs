fn a() {
    if {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn b() {
    if true && {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn c() {
    let x = {};
    if true x
// { dg-error "" "" { target *-*-* } .-1 }
}

fn a2() {
    if {} else {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn b2() {
    if true && {} else {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn c2() {
    let x = {};
    if true x else {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn d() {
    if true else {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


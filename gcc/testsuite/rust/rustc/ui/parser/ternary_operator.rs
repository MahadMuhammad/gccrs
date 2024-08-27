fn a() {
    let x = 5 > 2 ? true : false;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn b() {
    let x = 5 > 2 ? { true } : { false };
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn c() {
    let x = 5 > 2 ? f32::MAX : f32::MIN;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn bad() {
    // regression test for #117208
    v ? return;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let x = 5 > 2 ? { let x = vec![]: Vec<u16>; x } : { false };
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}


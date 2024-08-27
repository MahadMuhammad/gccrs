// check that we can't do funny things with wildcards.

fn a() {
    match 1 {
        _ + 1 => () // { dg-error "" "" { target *-*-* } }
    }
}

fn b() {
    match 2 {
        (_ % 4) => () // { dg-error "" "" { target *-*-* } }
    }
}

fn c() {
    match 3 {
        _.x() => () // { dg-error "" "" { target *-*-* } }
    }
}

fn d() {
    match 4 {
        _..=4 => () // { dg-error "" "" { target *-*-* } }
    }
}

fn e() {
    match 5 {
        .._ => () // { dg-error "" "" { target *-*-* } }
    }
}

fn f() {
    match 6 {
        0..._ => ()
// { dg-error ".E0586." "" { target *-*-* } .-1 }
// { dg-error ".E0586." "" { target *-*-* } .-2 }
    }
}

fn g() {
    match 7 {
        (_ * 0)..5 => () // { dg-error "" "" { target *-*-* } }
    }
}

fn h() {
    match 8 {
        ..(_) => () // { dg-error "" "" { target *-*-* } }
    }
}

fn i() {
    match 9 {
        4..=(2 + _) => ()
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

fn main() {}


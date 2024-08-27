// Check that suggestions to add a zero to integers with a preceding dot only appear when the change
// will result in a valid floating point literal.

fn main() {}

fn a() {
    _ = .3u32;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn b() {
    _ = .0b0;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn c() {
    _ = .0o07;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn d() {
    _ = .0x0ABC;
// { dg-error "" "" { target *-*-* } .-1 }
}


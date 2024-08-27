fn a() {
    if let x = 1 && i = 2 {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}

fn b() {
    if (i + j) = i {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
// { dg-error ".E0425." "" { target *-*-* } .-3 }
}

fn c() {
    if x[0] = 1 {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

fn main() {}


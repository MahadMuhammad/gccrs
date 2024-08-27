fn main() {
    let x = 42;
    match x {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-note ".E0004." "" { target *-*-* } .-2 }
// { dg-note ".E0004." "" { target *-*-* } .-3 }
        0..=73 => {},
        74..=> {},
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    }
}


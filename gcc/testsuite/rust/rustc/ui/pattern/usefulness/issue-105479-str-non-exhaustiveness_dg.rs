fn main() {
    let a = "";
    let b = "";
    match (a, b) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-note ".E0004." "" { target *-*-* } .-2 }
// { dg-note ".E0004." "" { target *-*-* } .-3 }
// { dg-note ".E0004." "" { target *-*-* } .-4 }
        ("a", "b") => {}
        ("c", "d") => {}
    }
}


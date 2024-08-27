// Test that we use fully-qualified type names in error messages.

fn main() {
    let x: // { dg-note "" "" { target *-*-* } }
        Option<usize>; // { dg-note "" "" { target *-*-* } }
    x = 5;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
}


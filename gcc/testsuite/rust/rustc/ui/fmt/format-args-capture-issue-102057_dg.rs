fn main() {
    format!("\x7Ba}");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    format!("\x7Ba\x7D");
// { dg-error ".E0425." "" { target *-*-* } .-1 }

    let a = 0;

    format!("\x7Ba} {b}");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    format!("\x7Ba\x7D {b}");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    format!("\x7Ba} \x7Bb}");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    format!("\x7Ba\x7D \x7Bb}");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    format!("\x7Ba\x7D \x7Bb\x7D");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}


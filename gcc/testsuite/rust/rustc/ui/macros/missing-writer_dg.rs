// Check error for missing writer in writeln! and write! macro
fn main() {
    let x = 1;
    let y = 2;
    write!("{}_{}", x, y);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
// { dg-error ".E0599." "" { target *-*-* } .-3 }
// { dg-note ".E0599." "" { target *-*-* } .-4 }
// { help ".E0599." "" { target *-*-* } .-5 }
    writeln!("{}_{}", x, y);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
// { dg-error ".E0599." "" { target *-*-* } .-3 }
// { dg-note ".E0599." "" { target *-*-* } .-4 }
// { help ".E0599." "" { target *-*-* } .-5 }
}


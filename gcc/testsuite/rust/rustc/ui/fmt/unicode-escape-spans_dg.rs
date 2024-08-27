fn main() {
    // 1 byte in UTF-8
    format!("\u{000041}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
    format!("\u{0041}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
    format!("\u{41}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
    format!("\u{0}{a}"); // { dg-error ".E0425." "" { target *-*-* } }

    // 2 bytes
    format!("\u{0df}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
    format!("\u{df}{a}"); // { dg-error ".E0425." "" { target *-*-* } }

    // 3 bytes
    format!("\u{00211d}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
    format!("\u{211d}{a}"); // { dg-error ".E0425." "" { target *-*-* } }

    // 4 bytes
    format!("\u{1f4a3}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
    format!("\u{10ffff}{a}"); // { dg-error ".E0425." "" { target *-*-* } }
}


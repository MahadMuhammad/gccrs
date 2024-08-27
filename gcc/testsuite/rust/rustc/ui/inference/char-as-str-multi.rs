// When a MULTI/NO-character string literal is used where a char should be,
// DO NOT suggest changing to single quotes.

fn main() {
    let _: char = "foo"; // { dg-error ".E0308." "" { target *-*-* } }
    let _: char = ""; // { dg-error ".E0308." "" { target *-*-* } }
}


// Verify that we do not ICE when the user uses a multubyte ampersand.

fn f(_: &＆()) -> &() { todo!() }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

fn main() {}


// Testing that we don't fail abnormally after hitting the errors

use unresolved::*;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-note ".E0432." "" { target *-*-* } .-2 }
// { help ".E0432." "" { target *-*-* } .-3 }

fn main() {}


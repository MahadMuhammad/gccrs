//@ normalize-stderr-test: "couldn't read.*" -> "couldn't read the file"

#![doc = include_str!("../not_existing_file.md")]
struct Documented {}
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}


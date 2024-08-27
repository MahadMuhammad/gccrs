// Test that `assert` works in consts.

const _: () = assert!(true);

const _: () = assert!(false);
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {}


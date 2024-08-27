// Checks that a const param cannot be stored in a struct.

struct S<const C: u8>(C); // { dg-error ".E0573." "" { target *-*-* } }

fn main() {}


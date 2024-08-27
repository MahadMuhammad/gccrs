use not_existing_crate::*; // { dg-error ".E0432." "" { target *-*-* } }
use std as foo;

fn main() {}


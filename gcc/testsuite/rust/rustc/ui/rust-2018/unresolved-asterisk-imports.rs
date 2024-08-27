// { dg-additional-options "-frust-edition=2018" }

use not_existing_crate::*; // { dg-error ".E0432." "" { target *-*-* } }
use std as foo;

fn main() {}


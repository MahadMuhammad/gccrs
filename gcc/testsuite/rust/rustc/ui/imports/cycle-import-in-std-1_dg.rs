// { dg-additional-options "-frust-edition= 2018" }

// https://github.com/rust-lang/rust/issues/124490

use ops::{self as std};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
use std::collections::{self as ops};

fn main() {}


// { dg-additional-options "-frust-edition= 2018" }

// https://github.com/rust-lang/rust/issues/125013

use ops::{self as std};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
use std::ops::Deref::{self as ops};

fn main() {}


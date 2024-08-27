// { dg-additional-options "-frust-edition=2018" }

use alloc::rc::Rc; // { dg-error ".E0433." "" { target *-*-* } }

fn main() {}


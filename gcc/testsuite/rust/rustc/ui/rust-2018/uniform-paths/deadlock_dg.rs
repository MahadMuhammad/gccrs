// { dg-additional-options "-frust-edition=2018" }
//@ compile-flags:--extern foo --extern bar

use bar::foo; // { dg-error ".E0432." "" { target *-*-* } }
use foo::bar; // { dg-error ".E0463." "" { target *-*-* } }
// { dg-error ".E0432." "" { target *-*-* } .-2 }

fn main() {}


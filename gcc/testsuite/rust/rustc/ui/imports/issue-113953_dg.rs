// { dg-additional-options "-frust-edition= 2021" }
use u8 as imported_u8;
use unresolved as u8;
// { dg-error ".E0432." "" { target *-*-* } .-1 }

fn main() {}


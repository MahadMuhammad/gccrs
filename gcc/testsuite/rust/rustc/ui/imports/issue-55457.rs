use NonExistent; // { dg-error ".E0432." "" { target *-*-* } }
use non_existent::non_existent; // { dg-error ".E0432." "" { target *-*-* } }

#[non_existent]
#[derive(NonExistent)]

struct S;

fn main() {}


use clippy::a; // { dg-error ".E0432." "" { target *-*-* } }
use clippy::a::b; // { dg-error ".E0433." "" { target *-*-* } }

use rustdoc::a; // { dg-error ".E0432." "" { target *-*-* } }
use rustdoc::a::b; // { dg-error ".E0433." "" { target *-*-* } }

fn main() {}


// { dg-additional-options "-frust-edition= 2018" }

use clippy::time::Instant;
// { dg-error ".E0433." "" { target *-*-* } .-1 }

fn main() {
    Instant::now();
}


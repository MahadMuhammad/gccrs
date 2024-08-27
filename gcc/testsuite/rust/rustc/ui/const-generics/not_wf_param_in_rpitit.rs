// { dg-additional-options "-frust-edition=2021" }

trait Trait<const N: Trait = bar> {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
// { dg-error ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }
// { dg-error ".E0782." "" { target *-*-* } .-5 }
// { dg-error ".E0782." "" { target *-*-* } .-6 }
// { dg-error ".E0782." "" { target *-*-* } .-7 }
    async fn a() {}
}

fn main() {}


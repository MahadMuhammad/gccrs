// { dg-additional-options "-frust-edition=2021" }

trait X {
    fn test() -> Self::Assoc<{ async {} }>;
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }

}

pub fn main() {}


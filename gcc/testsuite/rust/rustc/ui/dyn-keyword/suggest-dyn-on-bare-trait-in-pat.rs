// { dg-additional-options "-frust-edition= 2021" }

trait Trait {}

impl dyn Trait {
    const CONST: () = ();
}

fn main() {
    match () {
        Trait::CONST => {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
    }
}


// Ensure that the compiler include the blanklet implementation suggestion
// when inside a `impl` statement are used two local traits.
//
// { dg-additional-options "-frust-edition=2021" }
use std::fmt;

trait LocalTraitOne { }

trait LocalTraitTwo { }

trait GenericTrait<T> {}

impl LocalTraitTwo for LocalTraitOne {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }

impl fmt::Display for LocalTraitOne {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl fmt::Display for LocalTraitTwo + Send {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl LocalTraitOne for fmt::Display {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }


impl LocalTraitOne for fmt::Display + Send {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }


impl<E> GenericTrait<E> for LocalTraitOne {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }

trait GenericTraitTwo<T> {}

impl<T, E> GenericTraitTwo<E> for GenericTrait<T> {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }

fn main() {}


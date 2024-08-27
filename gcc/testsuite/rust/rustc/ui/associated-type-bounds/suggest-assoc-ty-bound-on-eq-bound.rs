// Regression test for issue #105056.
// { dg-additional-options "-frust-edition= 2021" }

fn f(_: impl Trait<T = Copy>) {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }

fn g(_: impl Trait<T = std::fmt::Debug + Eq>) {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }
// { help ".E0782." "" { target *-*-* } .-5 }
// { dg-error ".E0782." "" { target *-*-* } .-6 }

fn h(_: impl Trait<T<> = 'static + for<'a> Fn(&'a ())>) {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }
// { help ".E0782." "" { target *-*-* } .-3 }

// Don't suggest assoc ty bound in trait object types, that's not valid:
type Obj = dyn Trait<T = Clone>;
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { help ".E0782." "" { target *-*-* } .-2 }

trait Trait { type T; }

fn main() {}


use std::ops::Deref;
trait Foo {
    type Bar<'a>: Deref<Target = <Self>::Bar<Target = Self>>;
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }
// { dg-error ".E0229." "" { target *-*-* } .-3 }
// { help ".E0229." "" { target *-*-* } .-4 }
// { dg-error ".E0229." "" { target *-*-* } .-5 }
// { help ".E0229." "" { target *-*-* } .-6 }
// { dg-error ".E0229." "" { target *-*-* } .-7 }
// { help ".E0229." "" { target *-*-* } .-8 }
}

fn main() {}


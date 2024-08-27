// Regression test for #116464
// Checks that we do not suggest Trait<..., Assoc=arg> when the trait
// is referred to from one of its impls but do so at all other places

pub trait Trait<T> {
    type Assoc;
}

impl<T, S> Trait<T> for i32 {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type Assoc = String;
}

// Should not trigger suggestion here...
impl<T, S> Trait<T, S> for () {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }

//... but should do so in all of the below cases except the last one
fn func<T: Trait<u32, String>>(t: T) -> impl Trait<(), i32> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
    3
}

struct Struct<T: Trait<u32, String>> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    a: T
}

trait AnotherTrait<T: Trait<T, i32>> {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }

impl<T: Trait<u32, String>> Struct<T> {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }

// Test for self type. Should not trigger suggestion as it doesn't have an
// associated type
trait YetAnotherTrait {}
impl<T: Trait<u32, Assoc=String>, U> YetAnotherTrait for Struct<T, U> {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }


fn main() {
}


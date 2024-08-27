// issue #101376

use std::ops::AddAssign;
struct Foo;

impl AddAssign<()> for Foo {
    fn add_assign(&mut self, _: ()) {}
}

impl AddAssign<()> for &mut Foo {
    fn add_assign(&mut self, _: ()) {}
}

fn main() {
    (&mut Foo) += ();
// { dg-error ".E0067." "" { target *-*-* } .-1 }
// { dg-note ".E0067." "" { target *-*-* } .-2 }
// { help ".E0067." "" { target *-*-* } .-3 }
}


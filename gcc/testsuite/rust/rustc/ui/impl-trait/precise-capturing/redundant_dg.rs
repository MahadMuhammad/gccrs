//@ compile-flags: -Zunstable-options --edition=2024
//@ revisions: normal rpitit
//@[normal] check-pass

fn hello<'a>() -> impl Sized + use<'a> {}
// { dg-warning "" "" { target *-*-* } .-1 }

struct Inherent;
impl Inherent {
    fn inherent(&self) -> impl Sized + use<'_> {}
// { dg-warning "" "" { target *-*-* } .-1 }
}

#[cfg(rpitit)]
trait Test<'a> {
    fn in_trait() -> impl Sized + use<'a, Self>;
// { dg-error "" "" { target *-*-* } .-1 }
}
#[cfg(rpitit)]
impl<'a> Test<'a> for () {
    fn in_trait() -> impl Sized + use<'a> {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}


//@ revisions: real pre_expansion
//@[pre_expansion] check-pass
// { dg-additional-options "-frust-edition= 2021" }

#[cfg(real)]
trait Foo: use<> {
// { dg-error "" "" { target *-*-* } .-1 }
    type Assoc: use<> where (): use<>;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

#[cfg(real)]
fn fun<T: use<>>(_: impl use<>) where (): use<> {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

#[cfg(real)]
fn dynamic() -> Box<dyn use<>> {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}


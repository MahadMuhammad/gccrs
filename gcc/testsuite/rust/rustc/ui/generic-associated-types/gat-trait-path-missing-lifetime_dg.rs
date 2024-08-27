trait X {
  type Y<'a>;

  fn foo<'a>(t : Self::Y<'a>) -> Self::Y<'a> { t }
}

impl<T> X for T { // { dg-error ".E0046." "" { target *-*-* } }
  fn foo<'a, T1: X<Y = T1>>(t : T1) -> T1::Y<'a> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0049." "" { target *-*-* } .-2 }
    t
  }
}

fn main() {}


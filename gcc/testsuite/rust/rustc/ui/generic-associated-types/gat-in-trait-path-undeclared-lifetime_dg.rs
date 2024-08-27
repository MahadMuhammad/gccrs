trait X {
  type Y<'x>;
}

fn main() {
  fn _f(arg : Box<dyn for<'a> X<Y<'x> = &'a [u32]>>) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }
}


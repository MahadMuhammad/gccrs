trait X {
  type Y<'a>;
}

fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }
// { dg-error ".E0038." "" { target *-*-* } .-6 }
// { dg-error ".E0038." "" { target *-*-* } .-7 }
// { dg-error ".E0038." "" { target *-*-* } .-8 }
// { dg-error ".E0038." "" { target *-*-* } .-9 }
// { dg-error ".E0038." "" { target *-*-* } .-10 }


fn bar<'a>(arg: Box<dyn X<Y() = ()>>) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }

fn main() {}


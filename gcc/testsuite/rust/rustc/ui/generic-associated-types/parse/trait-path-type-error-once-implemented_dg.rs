trait X {
    type Y<'a>;
}

const _: () = {
  fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }
// { dg-error ".E0038." "" { target *-*-* } .-6 }
// { dg-error ".E0038." "" { target *-*-* } .-7 }
};

fn main() {}


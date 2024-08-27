trait X {
    type Y<'a>;
}

fn f(x: Box<dyn X<Y<'a> = &'a ()>>) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }

fn main() {}


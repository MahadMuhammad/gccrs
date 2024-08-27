trait X {
    type Y<'a, 'b>;
}

struct Foo<'a, 'b, 'c> {
    a: &'a u32,
    b: &'b str,
    c: &'c str,
}

fn foo<'c, 'd>(_arg: Box<dyn X<Y = (&'c u32, &'d u32)>>) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }

fn bar<'a, 'b, 'c>(_arg: Foo<'a, 'b>) {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }

fn f<'a>(_arg: Foo<'a>) {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }

fn main() {}


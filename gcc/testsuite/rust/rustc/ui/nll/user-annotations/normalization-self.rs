//@ check-fail

trait Trait { type Assoc; }
impl<'a> Trait for &'a () { type Assoc = &'a (); }

struct MyTuple<T>(T);
impl MyTuple<<&'static () as Trait>::Assoc> {
    fn test(x: &(), y: &()) {
        Self(x);
// { dg-error "" "" { target *-*-* } .-1 }
        let _: Self = MyTuple(y);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

struct MyStruct<T> { val: T, }
impl MyStruct<<&'static () as Trait>::Assoc> {
    fn test(x: &(), y: &()) {
        Self { val: x };
// { dg-error "" "" { target *-*-* } .-1 }
        let _: Self = MyStruct { val: y };
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}


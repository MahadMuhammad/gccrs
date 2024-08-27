pub trait X {
    type Y<'a> where Self: 'a;
    fn m(&self) -> Self::Y<'_>;
}

impl X for () {
    type Y<'a> = &'a ();

    fn m(&self) -> Self::Y<'_> {
        self
    }
}

fn f(x: &impl for<'a> X<Y<'a> = &'a ()>) -> &'static () {
    x.m()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn g<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &'static () {
    x.m()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn h(x: &()) -> &'static () {
    x.m()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    f(&());
    g(&());
    h(&());
}


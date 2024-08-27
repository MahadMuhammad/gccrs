//@ run-rustfix
#![allow(unused)]
struct S;

impl S {
    fn call(&mut self, f: impl FnOnce((), &mut Self)) {
        // change state or something ...
        f((), self);
        // change state or something ...
    }

    fn get(&self) {}
    fn set(&mut self) {}
}

fn main() {
    let mut v = S;

    v.call(|(), this: &mut S| v.get());
// { dg-error ".E0502." "" { target *-*-* } .-1 }
    v.call(|(), this: &mut S| v.set());
// { dg-error ".E0499." "" { target *-*-* } .-1 }
// { dg-error ".E0499." "" { target *-*-* } .-2 }

    v.call(|(), this: &mut S| {
// { dg-error ".E0499." "" { target *-*-* } .-1 }
// { dg-error ".E0499." "" { target *-*-* } .-2 }

        _ = v;
        v.set();
        v.get();
        S::get(&v);

        use std::ops::Add;
        let v = 0u32;
        _ = v + v;
        _ = v.add(3);
    });
}


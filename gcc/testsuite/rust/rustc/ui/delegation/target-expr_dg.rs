#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    fn static_method(x: i32) -> i32 { x }
}

struct F;

struct S(F);
impl Trait for S {}

fn foo(x: i32) -> i32 { x }

fn bar<T: Default>(_: T) {
    reuse Trait::static_method {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _ = T::Default();
// { dg-error ".E0401." "" { target *-*-* } .-1 }
    }
}

fn main() {
    let y = 0;
    reuse <S as Trait>::static_method {
        let x = y;
// { dg-error ".E0434." "" { target *-*-* } .-1 }
        foo(self);

        let reuse_ptr: fn(i32) -> i32  = static_method;
        reuse_ptr(0)
    }
    self.0;
// { dg-error ".E0424." "" { target *-*-* } .-1 }
    let z = x;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}


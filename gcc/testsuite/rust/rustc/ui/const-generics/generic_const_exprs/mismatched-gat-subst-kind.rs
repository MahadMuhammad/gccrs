#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait B {
    type U<T>;
}

fn f<T: B<U<1i32> = ()>>() {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }

fn main() {}


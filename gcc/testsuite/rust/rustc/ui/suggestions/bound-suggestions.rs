//@ run-rustfix

#[allow(unused)]
use std::fmt::Debug;
// Rustfix should add this, or use `std::fmt::Debug` instead.

#[allow(dead_code)]
fn test_impl(t: impl Sized) {
    println!("{:?}", t);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
fn test_no_bounds<T>(t: T) {
    println!("{:?}", t);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
fn test_one_bound<T: Sized>(t: T) {
    println!("{:?}", t);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
fn test_no_bounds_where<X, Y>(x: X, y: Y) where X: std::fmt::Debug, {
    println!("{:?} {:?}", x, y);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
fn test_one_bound_where<X>(x: X) where X: Sized {
    println!("{:?}", x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
fn test_many_bounds_where<X>(x: X) where X: Sized, X: Sized {
    println!("{:?}", x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
trait Foo<T> {
    const SIZE: usize = core::mem::size_of::<Self>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
trait Bar: std::fmt::Display {
    const SIZE: usize = core::mem::size_of::<Self>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
trait Baz where Self: std::fmt::Display {
    const SIZE: usize = core::mem::size_of::<Self>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
trait Qux<T> where Self: std::fmt::Display {
    const SIZE: usize = core::mem::size_of::<Self>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

#[allow(dead_code)]
trait Bat<T>: std::fmt::Display {
    const SIZE: usize = core::mem::size_of::<Self>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() { }


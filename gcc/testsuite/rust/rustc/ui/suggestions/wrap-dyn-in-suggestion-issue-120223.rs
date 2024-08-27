#![feature(dyn_star)] // { dg-warning "" "" { target *-*-* } }

use std::future::Future;

pub fn dyn_func<T>(
    executor: impl FnOnce(T) -> dyn Future<Output = ()>,
) -> Box<dyn FnOnce(T) -> dyn Future<Output = ()>> {
    Box::new(executor) // { dg-error ".E0310." "" { target *-*-* } }
}

pub fn dyn_star_func<T>(
    executor: impl FnOnce(T) -> dyn* Future<Output = ()>,
) -> Box<dyn FnOnce(T) -> dyn* Future<Output = ()>> {
    Box::new(executor) // { dg-error ".E0310." "" { target *-*-* } }
}

trait Trait {
    fn method(&self) {}
}

impl Trait for fn() {}

pub fn in_ty_param<T: Fn() -> dyn std::fmt::Debug> (t: T) {
    t.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn with_sized<T: Fn() -> &'static (dyn std::fmt::Debug) + ?Sized>() {
    without_sized::<T>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn without_sized<T: Fn() -> &'static dyn std::fmt::Debug>() {}

fn main() {}


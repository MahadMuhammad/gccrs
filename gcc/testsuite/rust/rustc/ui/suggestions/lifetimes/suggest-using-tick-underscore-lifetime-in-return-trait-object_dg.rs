//@ run-rustfix
use std::any::Any;

fn foo<T: Any>(value: &T) -> Box<dyn Any> {
    Box::new(value) as Box<dyn Any>
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let _ = foo(&5);
}


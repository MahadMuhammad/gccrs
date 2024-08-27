//@ run-rustfix

macro_rules! my_wrapper {
    ($expr:expr) => { MyWrapper($expr) }
}

pub struct MyWrapper(#[allow(dead_code)] u32);

fn main() {
    let value = MyWrapper(123);
    some_fn(value.0); // { dg-error ".E0308." "" { target *-*-* } }
    some_fn(my_wrapper!(123).0); // { dg-error ".E0308." "" { target *-*-* } }
}

fn some_fn(wrapped: MyWrapper) {
    drop(wrapped);
}


#![feature(tuple_trait)]

fn assert_is_tuple<T: std::marker::Tuple + ?Sized>() {}

struct TupleStruct(i32, i32);

fn from_param_env<T>() {
    assert_is_tuple::<T>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {
    assert_is_tuple::<i32>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    assert_is_tuple::<(i32)>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    assert_is_tuple::<TupleStruct>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


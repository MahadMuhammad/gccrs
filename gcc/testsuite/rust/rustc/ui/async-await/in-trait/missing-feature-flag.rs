// { dg-additional-options "-frust-edition=2018" }

#![feature(min_specialization)]

struct MyStruct;

trait MyTrait<T> {
    async fn foo(_: T) -> &'static str;
}

impl<T> MyTrait<T> for MyStruct {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

impl MyTrait<i32> for MyStruct {
    async fn foo(_: i32) -> &'static str {}
// { dg-error ".E0520." "" { target *-*-* } .-1 }
// { dg-error ".E0520." "" { target *-*-* } .-2 }
}

fn main() {}


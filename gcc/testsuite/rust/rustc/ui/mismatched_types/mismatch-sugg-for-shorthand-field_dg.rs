//@ run-rustfix
// { dg-additional-options "-frust-edition=2021" }
#![allow(dead_code)]
#![allow(unused_variables)]
use std::future::Future;
use std::pin::Pin;

fn test1() {
    let string = String::from("Hello, world");

    struct Demo<'a> {
        option: Option<&'a str>,
    }

    let option: Option<String> = Some(string.clone());
    let s = Demo { option }; // { dg-error ".E0308." "" { target *-*-* } }
}

fn test2() {
    let string = String::from("Hello, world");

    struct Demo<'a> {
        option_ref: Option<&'a str>,
    }

    let option_ref = Some(&string);
    let s = Demo { option_ref }; // { dg-error ".E0308." "" { target *-*-* } }
}

fn test3() {
    let string = String::from("Hello, world");

    struct Demo<'a> {
        option_ref_ref: Option<&'a str>,
    }

    let option_ref = Some(&string);
    let option_ref_ref = option_ref.as_ref();

    let s = Demo { option_ref_ref }; // { dg-error ".E0308." "" { target *-*-* } }
}

fn test4() {
    let a = 1;
    struct Demo {
        a: String,
    }
    let s = Demo { a }; // { dg-error ".E0308." "" { target *-*-* } }
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
fn test5() {
    let a = async { 42 };
    struct Demo {
        a: BoxFuture<'static, i32>,
    }
    let s = Demo { a }; // { dg-error ".E0308." "" { target *-*-* } }
}

fn test6() {
    struct A;
    struct B;

    impl From<B> for A {
        fn from(_: B) -> Self {
            A
        }
    }

    struct Demo {
        a: A,
    }
    let a = B;
    let s = Demo { a }; // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {}


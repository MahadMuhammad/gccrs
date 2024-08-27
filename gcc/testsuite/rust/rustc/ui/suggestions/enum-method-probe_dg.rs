//@ compile-flags: --edition=2021
//@ run-rustfix

#![allow(unused)]

struct Foo;

impl Foo {
    fn get(&self) -> u8 {
        42
    }
}

fn test_result_in_result() -> Result<(), ()> {
    let res: Result<_, ()> = Ok(Foo);
    res.get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    Ok(())
}

async fn async_test_result_in_result() -> Result<(), ()> {
    let res: Result<_, ()> = Ok(Foo);
    res.get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    Ok(())
}

fn test_result_in_unit_return() {
    let res: Result<_, ()> = Ok(Foo);
    res.get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

async fn async_test_result_in_unit_return() {
    let res: Result<_, ()> = Ok(Foo);
    res.get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

fn test_option_in_option() -> Option<()> {
    let res: Option<_> = Some(Foo);
    res.get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    Some(())
}

fn test_option_in_unit_return() {
    let res: Option<_> = Some(Foo);
    res.get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

fn main() {}


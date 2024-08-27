// { dg-additional-options "-frust-edition=2021" }

trait Foo {
    type Clone;
    fn foo() -> Clone;
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { help ".E0038." "" { target *-*-* } .-2 }
}

trait DbHandle: Sized {}

trait DbInterface {
    type DbHandle;
    fn handle() -> DbHandle;
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { help ".E0038." "" { target *-*-* } .-2 }
}

fn main() {}


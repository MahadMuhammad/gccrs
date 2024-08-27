trait Foo {
    type Clone;
    fn foo() -> Clone;
// { dg-warning ".E0038." "" { target *-*-* } .-1 }
// { dg-warning ".E0038." "" { target *-*-* } .-2 }
// { help ".E0038." "" { target *-*-* } .-3 }
// { dg-warning ".E0038." "" { target *-*-* } .-4 }
// { dg-warning ".E0038." "" { target *-*-* } .-5 }
// { help ".E0038." "" { target *-*-* } .-6 }
// { dg-error ".E0038." "" { target *-*-* } .-7 }
// { help ".E0038." "" { target *-*-* } .-8 }
}

trait DbHandle: Sized {}

trait DbInterface {
    type DbHandle;
    fn handle() -> DbHandle;
// { dg-warning ".E0038." "" { target *-*-* } .-1 }
// { dg-warning ".E0038." "" { target *-*-* } .-2 }
// { help ".E0038." "" { target *-*-* } .-3 }
// { dg-warning ".E0038." "" { target *-*-* } .-4 }
// { dg-warning ".E0038." "" { target *-*-* } .-5 }
// { help ".E0038." "" { target *-*-* } .-6 }
// { dg-error ".E0038." "" { target *-*-* } .-7 }
// { help ".E0038." "" { target *-*-* } .-8 }
}

fn main() {}


#![allow(dead_code)]

// Test that we report an error for unused type parameters in types and traits,
// and that we offer a helpful suggestion.

struct SomeStruct<A> { x: u32 }
// { dg-error ".E0392." "" { target *-*-* } .-1 }

enum SomeEnum<A> { Nothing }
// { dg-error ".E0392." "" { target *-*-* } .-1 }

// Here T might *appear* used, but in fact it isn't.
enum ListCell<T> {
    Cons(Box<ListCell<T>>),
// { dg-error "" "" { target *-*-* } .-1 }
    Nil
}

struct SelfTyAlias<T>(Box<Self>);
// { dg-error "" "" { target *-*-* } .-1 }

struct WithBounds<T: Sized> {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }

struct WithWhereBounds<T> where T: Sized {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }

struct WithOutlivesBounds<T: 'static> {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }

struct DoubleNothing<T> {
// { dg-error ".E0392." "" { target *-*-* } .-1 }
    s: SomeStruct<T>,
}

fn main() {}


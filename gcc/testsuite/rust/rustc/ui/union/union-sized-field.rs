use std::mem::ManuallyDrop;

union Foo<T: ?Sized> {
    value: ManuallyDrop<T>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct Foo2<T: ?Sized> {
    value: ManuallyDrop<T>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    t: u32,
}

enum Foo3<T: ?Sized> {
    Value(ManuallyDrop<T>),
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}


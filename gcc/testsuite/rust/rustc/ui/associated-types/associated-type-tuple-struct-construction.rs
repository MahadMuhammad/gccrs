// Users cannot yet construct structs through associated types
// in both expressions and patterns

#![feature(more_qualified_paths)]

fn main() {
    let <Foo as A>::Assoc(n) = <Foo as A>::Assoc(2);
// { dg-error ".E0575." "" { target *-*-* } .-1 }
// { dg-error ".E0575." "" { target *-*-* } .-2 }
    assert!(n == 2);
}

struct TupleStruct(i8);

struct Foo;


trait A {
    type Assoc;
}

impl A for Foo {
    type Assoc = TupleStruct;
}


fn main() {
    // destructure through a qualified path
    let <Foo as A>::Assoc { br } = StructStruct { br: 2 };
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    let _ = <Foo as A>::Assoc { br: 2 };
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    let <E>::V(..) = E::V(0);
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

struct StructStruct {
    br: i8,
}

struct Foo;

trait A {
    type Assoc;
}

impl A for Foo {
    type Assoc = StructStruct;
}

enum E {
    V(u8)
}


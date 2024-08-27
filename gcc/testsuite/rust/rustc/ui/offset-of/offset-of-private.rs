#![feature(offset_of_enum)]

use std::mem::offset_of;

mod m {
    #[repr(C)]
    pub struct Foo {
        pub public: u8,
        private: u8,
    }

    #[repr(C)]
    pub struct FooTuple(pub u8, u8);

    #[repr(C)]
    struct Bar {
        pub public: u8,
        private: u8,
    }

    pub enum Baz {
        Var1(Foo),
        Var2(u64),
    }
}

fn main() {
    offset_of!(m::Foo, public);
    offset_of!(m::Foo, private); // { dg-error ".E0616." "" { target *-*-* } }
    offset_of!(m::FooTuple, 0);
    offset_of!(m::FooTuple, 1); // { dg-error ".E0616." "" { target *-*-* } }
    offset_of!(m::Bar, public); // { dg-error ".E0603." "" { target *-*-* } }
    offset_of!(m::Bar, private); // { dg-error ".E0616." "" { target *-*-* } }
// { dg-error ".E0616." "" { target *-*-* } .-2 }

    offset_of!(m::Baz, Var1.0.public);
    offset_of!(m::Baz, Var1.0.private); // { dg-error ".E0616." "" { target *-*-* } }
    offset_of!(m::Baz, Var2.0);
}


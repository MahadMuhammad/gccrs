#![feature(type_alias_impl_trait)]

mod my_mod {
    use std::fmt::Debug;

    pub type Foo = impl Debug;
    pub type Foot = impl Debug;

    pub fn get_foo() -> Foo {
        5i32
    }

    pub fn get_foot(_: Foo) -> Foot {
// { dg-error "" "" { target *-*-* } .-1 }
        get_foo() // { dg-error "" "" { target *-*-* } }
    }
}

fn main() {
    let _: my_mod::Foot = my_mod::get_foot(my_mod::get_foo());
}


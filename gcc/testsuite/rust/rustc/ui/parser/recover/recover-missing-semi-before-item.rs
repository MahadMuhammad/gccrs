//@ run-rustfix

#![allow(unused_variables, dead_code, unused_imports)]

fn for_struct() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    struct Foo;
}

fn for_union() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    union Foo {
        foo: usize,
    }
}

fn for_enum() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    enum Foo {
        Bar,
    }
}

fn for_fn() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    fn foo() {}
}

fn for_extern() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    extern fn foo() {}
}

fn for_impl() {
    struct Foo;
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    impl Foo {}
}

fn for_use() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    pub use bar::Bar;
}

fn for_mod() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    mod foo {}
}

fn for_type() {
    let foo = 3 // { dg-error "" "" { target *-*-* } }
    type Foo = usize;
}

mod bar {
    pub struct Bar;
}

const X: i32 = 123 // { dg-error "" "" { target *-*-* } }

fn main() {}


#![feature(impl_trait_in_assoc_type)]

trait Foo {
    type Assoc;
    fn bar() -> Self::Assoc;
}

impl Foo for () {
    type Assoc = impl Sized;
    fn bar() -> Self::Assoc {
        fn foo() -> <() as Foo>::Assoc {
            let x: <() as Foo>::Assoc = 42_i32; // { dg-error ".E0308." "" { target *-*-* } }
            x
        };
        let _: i32 = foo();
        1i32
    }
}

fn main() {}


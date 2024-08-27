// test for ICE #121472 index out of bounds un_derefer.rs
#![feature(type_alias_impl_trait)]

mod foo {
    pub trait T {}

    pub type Alias<'a> = impl T;
    fn bar() {
        super::with_positive(|&n| ());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

use foo::*;

struct S;
impl<'a> T for &'a S {}

fn with_positive(fun: impl Fn(Alias<'_>)) {}

fn main() {}


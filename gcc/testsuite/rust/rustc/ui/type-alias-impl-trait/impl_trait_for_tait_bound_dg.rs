#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

type Foo = impl Debug;
pub trait Yay { }
impl Yay for Foo { }

fn foo() {
    is_yay::<u32>();   // { dg-error ".E0277." "" { target *-*-* } }
    is_debug::<u32>(); // OK
    is_yay::<Foo>();   // OK
    is_debug::<Foo>(); // OK
}

fn is_yay<T: Yay>() { }
fn is_debug<T: Debug>() { }

fn main() {}


#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

type Foo = impl Debug;

pub trait Yay { }
impl Yay for u32 { }

fn foo() {
    is_yay::<Foo>(); // { dg-error ".E0277." "" { target *-*-* } }
}

fn is_yay<T: Yay>() { }

fn main() {}


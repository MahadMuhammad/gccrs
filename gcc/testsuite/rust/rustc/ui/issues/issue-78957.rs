#![deny(unused_attributes)]

use std::marker::PhantomData;

pub struct Foo<#[inline] const N: usize>;
// { dg-error ".E0518." "" { target *-*-* } .-1 }
pub struct Bar<#[cold] const N: usize>;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
pub struct Baz<#[repr(C)] const N: usize>;
// { dg-error ".E0517." "" { target *-*-* } .-1 }
//
pub struct Foo2<#[inline] 'a>(PhantomData<&'a ()>);
// { dg-error ".E0518." "" { target *-*-* } .-1 }
pub struct Bar2<#[cold] 'a>(PhantomData<&'a ()>);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
pub struct Baz2<#[repr(C)] 'a>(PhantomData<&'a ()>);
// { dg-error ".E0517." "" { target *-*-* } .-1 }
//
pub struct Foo3<#[inline] T>(PhantomData<T>);
// { dg-error ".E0518." "" { target *-*-* } .-1 }
pub struct Bar3<#[cold] T>(PhantomData<T>);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
pub struct Baz3<#[repr(C)] T>(PhantomData<T>);
// { dg-error ".E0517." "" { target *-*-* } .-1 }

fn main() {}


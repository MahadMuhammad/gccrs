#![feature(allocator_api)]
#![feature(const_trait_impl)]

use core::convert::{From, TryFrom};
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }

use std::pin::Pin;
use std::alloc::Allocator;
impl<T: ?Sized, A: Allocator> const From<Box<T, A>> for Pin<Box<T, A>>
where
    A: 'static,
{}

pub fn main() {}


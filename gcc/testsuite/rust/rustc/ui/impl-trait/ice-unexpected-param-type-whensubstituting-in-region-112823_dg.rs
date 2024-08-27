// test for ICE #112823
// Unexpected parameter Type(Repr) when substituting in region

#![feature(impl_trait_in_assoc_type)]

use std::future::Future;

trait Stream {}

trait X {
    type LineStream<'a, Repr>
    where
        Self: 'a;
    type LineStreamFut<'a, Repr>
    where
        Self: 'a;
}

struct Y;

impl X for Y {
    type LineStream<'c, 'd> = impl Stream;
// { dg-error ".E0049." "" { target *-*-* } .-1 }
    type LineStreamFut<'a, Repr> = impl Future<Output = Self::LineStream<'a, Repr>>;
    fn line_stream<'a, Repr>(&'a self) -> Self::LineStreamFut<'a, Repr> {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

pub fn main() {}


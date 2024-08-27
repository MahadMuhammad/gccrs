#![feature(min_specialization)]
use std::fmt::{self, Display};

pub enum Cow<'a, B: ?Sized + 'a, O = <B as ToOwned>::Owned>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(O),
}

impl ToString for Cow<'_, str> {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl<B: ?Sized> Display for Cow<'_, B> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
        write!(f, "foo")
    }
}

fn main() {}


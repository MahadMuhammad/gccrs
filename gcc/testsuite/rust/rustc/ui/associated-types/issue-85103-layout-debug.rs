#![feature(rustc_attrs)]

use std::borrow::Cow;

#[rustc_layout(debug)]
type Edges<'a, E> = Cow<'a, [E]>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}


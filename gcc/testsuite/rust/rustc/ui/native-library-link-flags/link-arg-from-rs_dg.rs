#![feature(link_arg_attribute)]

#[link(kind = "link-arg", name = "arg", modifiers = "+bundle")]
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" {}

pub fn main() {}


use std::fmt::Debug;

pub fn foo<I: Iterator>(mut iter: I, value: &I::Item)
where
    I::Item: Eq + Debug,
{
    debug_assert_eq!(iter.next(), Some(value));
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}


// Makes sure we deal with escaping lifetimes *above* INNERMOST when
// suggesting trait for ambiguous associated type.

impl<I> Validator<I> for ()
where
    for<'iter> dyn Validator<<&'iter I>::Item>:,
// { dg-error ".E0223." "" { target *-*-* } .-1 }
{}

pub trait Validator<T> {}

fn main() {}


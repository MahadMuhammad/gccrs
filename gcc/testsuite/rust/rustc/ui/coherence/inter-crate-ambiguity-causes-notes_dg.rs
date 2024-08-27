//@ revisions: old next
//@[next] compile-flags: -Znext-solver

struct S;

impl From<()> for S {
    fn from(x: ()) -> Self {
        S
    }
}

impl<I> From<I> for S
// { dg-error "" "" { target *-*-* } .-1 }
where
    I: Iterator<Item = ()>,
{
    fn from(x: I) -> Self {
        S
    }
}

fn main() {}


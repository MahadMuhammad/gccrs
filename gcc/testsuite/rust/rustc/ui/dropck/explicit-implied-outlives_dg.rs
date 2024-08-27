//@ revisions: good1 good2 bad1 bad2
//@[good1] check-pass
//@[good2] check-pass

use std::ops::Drop;

struct DropMe<'a, T>(&'a T);

#[cfg(good1)]
impl<'a, T> Drop for DropMe<'a, T>
where
    T: 'a, // Implied by struct, explicit on impl
{
    fn drop(&mut self) {}
}

#[cfg(good2)]
impl<'a, T> Drop for DropMe<'a, T>
where
    'static: 'a, // Trivial bound
{
    fn drop(&mut self) {}
}

#[cfg(bad1)]
impl<'a, T> Drop for DropMe<'a, T>
where
    T: 'static,
// { dg-error "" "" { target *-*-* } .-1 }
{
    fn drop(&mut self) {}
}

#[cfg(bad2)]
impl<'a, T> Drop for DropMe<'a, T>
where
    'a: 'static,
// { dg-error "" "" { target *-*-* } .-1 }
{
    fn drop(&mut self) {}
}

fn main() {}


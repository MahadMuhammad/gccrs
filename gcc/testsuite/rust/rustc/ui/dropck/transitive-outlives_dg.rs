//@ revisions: good bad
//@[good] check-pass

use std::marker::PhantomData;
use std::ops::Drop;

struct DropMe<'a, 'b: 'a, 'c: 'b>(PhantomData<&'a ()>, PhantomData<&'b ()>, PhantomData<&'c ()>);

#[cfg(good)]
impl<'a, 'b, 'c> Drop for DropMe<'a, 'b, 'c>
where
    'c: 'a,
{
    fn drop(&mut self) {}
}

#[cfg(bad)]
impl<'a, 'b, 'c> Drop for DropMe<'a, 'b, 'c>
// { dg-error "" "" { target *-*-* } .-1 }
where
    'a: 'c,
{
    fn drop(&mut self) {}
}

fn main() {}


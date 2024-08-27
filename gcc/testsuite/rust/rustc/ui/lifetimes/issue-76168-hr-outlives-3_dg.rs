// { dg-additional-options "-frust-edition=2018" }

#![feature(unboxed_closures)]
use std::future::Future;

async fn wrapper<F>(f: F)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
where
F:,
for<'a> <i32 as FnOnce<(&'a mut i32,)>>::Output: Future<Output = ()> + 'a,
{
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let mut i = 41;
    &mut i;
}

fn main() {}


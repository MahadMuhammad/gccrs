#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait True {}

struct Is<const V: bool>;

impl True for Is<true> {}

fn g<T>()
// { dg-note "" "" { target *-*-* } .-1 }
where
    Is<{ std::mem::size_of::<T>() == 0 }>: True,
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
{
}

fn main() {
    g::<usize>();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
}


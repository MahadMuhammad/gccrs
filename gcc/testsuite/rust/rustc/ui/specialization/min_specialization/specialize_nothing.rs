#![feature(min_specialization)]

trait Special {
    fn be_special();
}

impl<T> Special for T {
    fn be_special() {}
}

impl Special for usize {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


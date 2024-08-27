use std::cell::Cell;

fn check<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>)
where
    'a: 'b,
{
}

fn test<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>) {
    let f = check;
// { dg-error "" "" { target *-*-* } .-1 }
    f(x, y);
}

fn main() {}


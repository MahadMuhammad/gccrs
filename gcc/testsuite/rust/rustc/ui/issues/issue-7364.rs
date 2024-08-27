use std::cell::RefCell;

// Regression test for issue 7364
static boxed: Box<RefCell<isize>> = Box::new(RefCell::new(0));
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }

fn main() { }


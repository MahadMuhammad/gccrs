static mut a: Box<isize> = Box::new(3);
// { dg-error ".E0015." "" { target *-*-* } .-1 }

fn main() {}


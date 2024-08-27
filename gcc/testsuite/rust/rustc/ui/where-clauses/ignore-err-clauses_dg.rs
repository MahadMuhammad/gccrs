use std::ops::Add;

fn dbl<T>(x: T) -> <T as Add>::Output
where
    T: Copy + Add,
    UUU: Copy,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{
    x + x
}

fn main() {
    println!("{}", dbl(3));
}


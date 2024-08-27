struct MyArray<const COUNT: usize>([u8; COUNT + 1]);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


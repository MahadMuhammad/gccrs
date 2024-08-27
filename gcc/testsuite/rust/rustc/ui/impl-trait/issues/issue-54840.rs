use std::ops::Add;

fn main() {
    let i: i32 = 0;
    let j: &impl Add = &i;
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}


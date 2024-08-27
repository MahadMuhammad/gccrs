use std::fmt::Debug;

fn main() {
    let x: Option<impl Debug> = Some(44_u32);
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    println!("{:?}", x);
}


//@ check-fail

#![feature(const_option)]

const FOO: i32 = Some(42i32).unwrap();

const BAR: i32 = Option::<i32>::None.unwrap();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {
    println!("{}", FOO);
    println!("{}", BAR);
}


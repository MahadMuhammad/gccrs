fn a(i: i32) -> i32 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let j = 2i32;
}

fn b(i: i32, j: i32) -> i32 {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn main() {}


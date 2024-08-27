const X: u32 = 5;
const Y: u32 = 6;
const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {
    println!("{}", FOO);
}


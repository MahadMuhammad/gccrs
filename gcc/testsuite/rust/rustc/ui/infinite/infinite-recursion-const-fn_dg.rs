//https://github.com/rust-lang/rust/issues/31364

const fn a() -> usize {
    b() // { dg-error ".E0080." "" { target *-*-* } }
}
const fn b() -> usize {
    a()
}
const ARR: [i32; a()] = [5; 6];

fn main() {}


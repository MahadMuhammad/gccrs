#[derive(Copy, Clone)]
pub struct Foo {
    x: [u8; SIZE],
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

const SIZE: u32 = 1;

fn main() {}


#[derive(Copy, Clone)]
struct Bar;

impl Copy for &'static mut Bar { }
// { dg-error ".E0206." "" { target *-*-* } .-1 }

fn main() {
}


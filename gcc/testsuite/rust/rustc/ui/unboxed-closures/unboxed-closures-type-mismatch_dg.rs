use std::ops::FnMut;

fn main() {
    let mut f = |x: isize, y: isize| -> isize { x + y };
    let z = f(1_usize, 2); // { dg-error ".E0308." "" { target *-*-* } }
    println!("{}", z);
    let mut g = |x, y| { x + y };
    let y = g(1_i32, 2);
    let z = g(1_usize, 2); // { dg-error ".E0308." "" { target *-*-* } }
    println!("{}", z);
}

trait T {
    fn bar(&self) {
        let identity = |x| x;
        identity(1u8);
        identity(1u16); // { dg-error ".E0308." "" { target *-*-* } }
        let identity = |x| x;
        identity(&1u8);
        identity(&1u16); // { dg-error ".E0308." "" { target *-*-* } }
    }
}

struct S;

impl T  for S {
    fn bar(&self) {
        let identity = |x| x;
        identity(1u8);
        identity(1u16); // { dg-error ".E0308." "" { target *-*-* } }
        let identity = |x| x;
        identity(&1u8);
        identity(&1u16); // { dg-error ".E0308." "" { target *-*-* } }
    }
}


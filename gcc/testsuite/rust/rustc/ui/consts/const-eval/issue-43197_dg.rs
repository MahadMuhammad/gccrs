const fn foo(x: u32) -> u32 {
    x
}

fn main() {
    const X: u32 = 0 - 1;
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    const Y: u32 = foo(0 - 1);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    println!("{} {}", X, Y);
}


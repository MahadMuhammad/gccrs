fn f() -> isize {
    let mut x: isize;
    for _ in 0..0 { x = 10; }
    return x; // { dg-error ".E0381." "" { target *-*-* } }
}

fn main() { f(); }


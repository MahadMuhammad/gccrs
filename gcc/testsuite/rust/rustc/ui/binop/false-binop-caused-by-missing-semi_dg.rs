//@ run-rustfix
fn foo() {}
fn main() {
    let mut y = 42;
    let x = &mut y;
    foo()
    *x = 0;  // { dg-error ".E0070." "" { target *-*-* } }
    let _ = x;
    println!("{y}");
}


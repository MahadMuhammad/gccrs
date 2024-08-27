fn bar() {}
fn foo(x: i32) -> u32 {
    0
}
fn main() {
    let b: fn() -> u32 = bar; // { dg-error ".E0308." "" { target *-*-* } }
    let f: fn(i32) = foo; // { dg-error ".E0308." "" { target *-*-* } }
}


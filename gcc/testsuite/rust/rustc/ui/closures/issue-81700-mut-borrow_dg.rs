fn foo(x: &mut u32) {
    let bar = || { foo(x); };
    bar(); // { dg-error ".E0596." "" { target *-*-* } }
}
fn main() {}


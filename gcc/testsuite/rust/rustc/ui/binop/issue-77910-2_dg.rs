fn foo(s: &i32) -> &i32 {
    let xs;
    xs // { dg-error ".E0381." "" { target *-*-* } }
}
fn main() {
    let y;
    if foo == y {}
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}


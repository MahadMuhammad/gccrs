fn foo(s: &i32) -> &i32 {
    let xs;
    xs // { dg-error ".E0381." "" { target *-*-* } }
}
fn main() {
    let y;
    // we shouldn't ice with the bound var here.
    assert_eq!(foo, y);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}


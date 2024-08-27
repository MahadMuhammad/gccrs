//@ run-rustfix
fn main() {
    let foo = &[1,2,3].iter();
    for _i in &foo {} // { dg-error ".E0277." "" { target *-*-* } }
}


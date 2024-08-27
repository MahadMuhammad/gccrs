fn foo(_: impl Iterator<Item = i32> + ?Sized) {} // { dg-error ".E0277." "" { target *-*-* } }
fn bar(_: impl ?Sized) {} // { dg-error ".E0277." "" { target *-*-* } }

fn main() {}


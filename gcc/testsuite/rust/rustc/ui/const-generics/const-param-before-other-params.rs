fn bar<const X: u8, 'a>(_: &'a ()) {
// { dg-error "" "" { target *-*-* } .-1 }
}

fn foo<const X: u8, T>(_: &T) {}

fn main() {}


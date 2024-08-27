fn foo<T>() where T: Default -> impl Default + 'static {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}


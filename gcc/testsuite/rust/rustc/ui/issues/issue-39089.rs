fn f<T: ?for<'a> Sized>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


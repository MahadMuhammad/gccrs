trait Bar {}
safe impl Bar for () { }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


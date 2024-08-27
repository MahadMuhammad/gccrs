fn foo(x: &mut Vec<&'_ u8>, y: &'_ u8) { x.push(y); }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}


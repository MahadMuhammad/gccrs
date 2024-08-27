fn test<T: !Copy>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

